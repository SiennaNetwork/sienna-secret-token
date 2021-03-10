use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, WasmMsg, CosmosMsg, log, HumanAddr
};
use amm_shared::{TokenPair, ExchangeInitMsg, ContractInfo, Callback};

use crate::msg::{InitMsg, HandleMsg, QueryMsg, QueryResponse};
use crate::state::{
    save_config, load_config, Config, pair_exists, store_exchange,
    get_address_for_pair, get_pair_for_address, Exchange
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let InitMsg {
        lp_token_contract,
        pair_contract,
        token_addr
    } = msg;

    let config = Config {
        lp_token_contract,
        pair_contract,
        token_addr
    };
    
    save_config(&mut deps.storage, &config)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreateExchange { pair } => create_exchange(deps, env, pair),
        HandleMsg::RegisterExchange { pair } => register_exchange(deps, env, pair)
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetExchangePair { exchange_addr } => query_exchange_pair(deps, exchange_addr),
        QueryMsg::GetPairExchangeAddress { pair } => query_exchange_address(deps, pair)
    }
}

fn create_exchange<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pair: TokenPair
) -> StdResult<HandleResponse> {

    if pair_exists(deps, &pair)? {
        return Err(StdError::generic_err("Pair already exists"));
    }

    let config = load_config(&deps.storage)?;

    // Actually creating the exchange happens when the instantiated contract calls
    // us back via the HandleMsg::RegisterExchange so that we can get its address.
    // This is also more robust as we should register the pair only if the exchange
    // contract has been successfully instantiated.

    Ok(HandleResponse{
        messages: vec![
            CosmosMsg::Wasm(
                WasmMsg::Instantiate {
                    code_id: config.pair_contract.id,
                    callback_code_hash: config.pair_contract.code_hash,
                    send: vec![],
                    label: format!(
                        "{}-{}-pair-{}-{}",
                        pair.0,
                        pair.1,
                        env.contract.address.clone(),
                        config.pair_contract.id
                    ),
                    msg: to_binary(
                        &ExchangeInitMsg {
                            pair: pair.clone(),
                            lp_token_contract: config.lp_token_contract.clone(),
                            factory_info: ContractInfo {
                                code_hash: env.contract_code_hash.clone(),
                                address: env.contract.address.clone()
                            },
                            callback: Callback {
                                contract_addr: env.contract.address,
                                contract_code_hash: env.contract_code_hash,
                                msg: to_binary(&HandleMsg::RegisterExchange {
                                    pair: pair.clone()
                                })?,
                            }
                        }
                    )?
                }
            )
        ],
        log: vec![
            log("action", "create_exchange"),
            log("pair", pair),
        ],
        data: None
    })
}

fn register_exchange<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    pair: TokenPair
) -> StdResult<HandleResponse> {
    let exchange = Exchange {
        pair: pair,
        address: env.message.sender
    };

    store_exchange(deps, &exchange)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "register_exchange"),
            log("pair", exchange.pair),
        ],
        data: None
    })
}

fn query_exchange_pair<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    exchange_addr: HumanAddr
) -> StdResult<Binary> {
    let pair = get_pair_for_address(deps, &exchange_addr)?;

    to_binary(&QueryResponse::GetExchangePair {
        pair
    })
}

fn query_exchange_address<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    pair: TokenPair
) -> StdResult<Binary> {
    let address = get_address_for_pair(deps, &pair)?;
    let address = deps.api.human_address(&address)?;
    
    to_binary(&QueryResponse::GetPairExchangeAddress {
        address
    })
}
