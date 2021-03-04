use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, ReadonlyStorage, QueryResult, CosmosMsg, WasmMsg
};
use secret_toolkit::snip20::{register_receive_msg, set_viewing_key_msg};
use amm_shared::{PairInitMsg, LpTokenInitMsg, TokenType, TokenPairAmount};
use utils::viewing_key::ViewingKey;

use crate::msg::{HandleMsg, HandleMsgResponse, QueryMsg, QueryMsgResponse, LiquidityDeposit};
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
        msg: to_binary(&LpTokenInitMsg {
            name: format!(
                "SecretSwap Liquidity Provider (LP) token for {}-{}",
                &msg.pair.0, &msg.pair.1
            ),
            admin: env.contract.address.clone(),
            symbol: "SWAP-LP".to_string(),
            decimals: 6
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
        lp_token_contract: msg.lp_token_contract.clone(),
        pair: msg.pair,
        contract_addr: env.contract.address
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
    deposit: LiquidityDeposit
) -> StdResult<HandleResponse> {
    let config = load_config(&deps.storage)?;

    if config.pair != deposit.pair {
        return Err(StdError::generic_err("The provided tokens dont match those managed by the contract."));
    }

    unimplemented!()
}

fn query_pool_amount<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> QueryResult {
    let config = load_config(&deps.storage)?;

    let result = config.pair.query_balances(deps, config.contract_addr)?;

    to_binary(&QueryMsgResponse::Pool(
        TokenPairAmount {
            pair: config.pair,
            amount_0: result.0,
            amount_1: result.1
        }
    ))
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
