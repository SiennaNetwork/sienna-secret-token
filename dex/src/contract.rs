use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, ReadonlyStorage, QueryResult, CosmosMsg, WasmMsg, Uint128, log, HumanAddr
};
use secret_toolkit::snip20::{register_receive_msg, set_viewing_key_msg, transfer_from_msg, token_info_query};
use amm_shared::{PairInitMsg, TokenInitMsg, TokenType, TokenPairAmount, ContractInfo, Callback};
use utils::viewing_key::ViewingKey;

use crate::msg::{HandleMsg, HandleMsgResponse, QueryMsg, QueryMsgResponse};
use crate::state::{Config, store_config, load_config};

/// Pad handle responses and log attributes to blocks
/// of 256 bytes to prevent leaking info based on response size
const BLOCK_SIZE: usize = 256;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: PairInitMsg,
) -> StdResult<InitResponse> {

    let mut messages = vec![];

    let viewing_key = ViewingKey::new(&env, b"YfhL28JtDN", b"JdjhIh3KhM");

    try_register_custom_token(&env, &mut messages, &msg.pair.0, &viewing_key)?;
    try_register_custom_token(&env, &mut messages, &msg.pair.1, &viewing_key)?;

    // Create LP token
    messages.push(CosmosMsg::Wasm(WasmMsg::Instantiate {
        code_id: msg.lp_token_contract.id,
        msg: to_binary(&TokenInitMsg {
            name: format!(
                "SecretSwap Liquidity Provider (LP) token for {}-{}",
                &msg.pair.0, &msg.pair.1
            ),
            admin: env.contract.address.clone(),
            symbol: "SWAP-LP".to_string(),
            decimals: 6,
            callback: Some(Callback {
                msg: to_binary(&HandleMsg::OnLpTokenInit)?,
                contract_addr: env.contract.address.clone(),
                contract_code_hash: env.contract_code_hash
            })
        })?,
        send: vec![],
        label: format!(
            "{}-{}-SecretSwap-LP-Token-{}",
            &msg.pair.0,
            &msg.pair.1,
            &env.contract.address.clone()
        ),
        callback_code_hash: msg.lp_token_contract.code_hash.clone(),
    }));

    let config = Config {
        factory_info: msg.factory_info,
        lp_token_info: ContractInfo {
            code_hash: msg.lp_token_contract.code_hash,
            // We get the address when the instantiated LP token calls OnLpTokenInit
            address: HumanAddr::default()
        },
        pair: msg.pair,
        contract_addr: env.contract.address,
        viewing_key: viewing_key
    };

    store_config(&mut deps.storage, &config)?;

    Ok(InitResponse {
        messages,
        log: vec![]
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddLiquidity { input } => add_liquidity(deps, env, input),
        HandleMsg::OnLpTokenInit => register_lp_token(deps, env),
        _ => unimplemented!()
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> QueryResult {
    let config = load_config(&deps.storage)?;

    match msg {
        QueryMsg::PairInfo => to_binary(&QueryMsgResponse::PairInfo(config.pair)),
        QueryMsg::FactoryInfo => to_binary(&QueryMsgResponse::FactoryInfo(config.factory_info)),
        QueryMsg::Pool => query_pool_amount(deps)
    }
}

fn add_liquidity<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    deposit: TokenPairAmount
) -> StdResult<HandleResponse> {

    let config = load_config(&deps.storage)?;

    let Config {
        pair,
        contract_addr,
        viewing_key,
        lp_token_info,
        ..
    } = config;
    
    if pair != deposit.pair {
        return Err(StdError::generic_err("The provided tokens dont match those managed by the contract."));
    }

    let mut messages: Vec<CosmosMsg> = vec![];

    let mut pool_balances = deposit.pair.query_balances(deps, contract_addr, viewing_key.0)?;
    let mut i = 0;

    for (amount, token) in deposit.into_iter() {
        match &token {
            TokenType::CustomToken { contract_addr, token_code_hash } => {
                messages.push(transfer_from_msg(
                    env.message.sender.clone(),
                    env.contract.address.clone(),
                    amount,
                    None,
                    BLOCK_SIZE,
                    token_code_hash.clone(),
                    contract_addr.clone())?
                );
            },
            TokenType::NativeToken { .. } => {
                pool_balances[i] = (pool_balances[i] - amount)?;
            }
        }

        i += 1;
    }

    let liquidity_supply = query_liquidity(&deps.querier, &lp_token_info)?;

    unimplemented!()
}

fn query_pool_amount<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> QueryResult {
    let config = load_config(&deps.storage)?;

    let result = config.pair.query_balances(deps, config.contract_addr, config.viewing_key.0)?;

    to_binary(&QueryMsgResponse::Pool(
        TokenPairAmount {
            pair: config.pair,
            amount_0: result[0],
            amount_1: result[1]
        }
    ))
}

fn query_liquidity(querier: &impl Querier, lp_token_info: &ContractInfo) -> StdResult<Uint128> {
    let result = token_info_query(
        querier,
        BLOCK_SIZE,
        lp_token_info.code_hash.clone(),
        lp_token_info.address.clone()
    )?;

    //If this happens, the LP token has been incorrectly configured
    if result.total_supply.is_none() {
        panic!("LP token has no available supply.");
    }

    Ok(result.total_supply.unwrap())
}

fn register_lp_token<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env
) -> StdResult<HandleResponse> {
    let mut config = load_config(&deps.storage)?;

    //This should only be set once when the LP token is instantiated.
    if config.lp_token_info.address != HumanAddr::default() {
        return Err(StdError::unauthorized());
    }

    config.lp_token_info.address = env.message.sender.clone();

    store_config(&mut deps.storage, &config)?;

    Ok(HandleResponse {
        messages: vec![register_receive_msg(
            env.contract_code_hash,
            None,
            BLOCK_SIZE,
            config.lp_token_info.code_hash,
            env.message.sender.clone(),
        )?],
        log: vec![log("liquidity_token_addr", env.message.sender.as_str())],
        data: None,
    })
}

fn try_register_custom_token(
    env: &Env,
    messages: &mut Vec<CosmosMsg>,
    token: &TokenType,
    viewing_key: &ViewingKey
) -> StdResult<()> {
    if let TokenType::CustomToken { 
        contract_addr, token_code_hash, ..
    } = token {
        messages.push(set_viewing_key_msg(
            viewing_key.0.clone(),
            None,
            BLOCK_SIZE,
            token_code_hash.clone(),
            contract_addr.clone(),
        )?);
        messages.push(register_receive_msg(
            env.contract_code_hash.clone(),
            None,
            BLOCK_SIZE,
            token_code_hash.clone(),
            contract_addr.clone(),
        )?);
    }

    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

}
*/
