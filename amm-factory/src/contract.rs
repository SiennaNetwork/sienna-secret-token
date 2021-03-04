use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, WasmMsg, CosmosMsg, log, HumanAddr
};
use amm_shared::{TokenPair, PairInitMsg, ContractInfo};

use crate::msg::{InitMsg, HandleMsg, QueryMsg, QueryResponse, Exchange};
use crate::state::{save_config, load_config, Config, pair_exists, store_exchange, get_address_for_pair, get_pair_for_address};

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
        HandleMsg::CreatePair { pair } => try_create_pair(deps, &env, pair),
        HandleMsg::RegisterExchange { exchange } => register_exchange(deps, exchange)
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

fn try_create_pair<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    pair: TokenPair
) -> StdResult<HandleResponse> {
    if pair_exists(deps, &pair)? {
        return Err(StdError::generic_err("Pair already exists"));
    }

    let config = load_config(&deps.storage)?;
    let log_msg = format!("{}-{}", pair.0, pair.1);

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
                        &PairInitMsg {
                            pair,
                            lp_token_contract: config.lp_token_contract.clone(),
                            factory_info: ContractInfo {
                                code_hash: env.contract_code_hash.clone(),
                                address: env.contract.address.clone()
                            }
                        }
                    )?
                }
            )
        ],
        log: vec![
            log("action", "create_pair"),
            log("pair", log_msg),
        ],
        data: None
    })
}

fn register_exchange<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    exchange: Exchange
) -> StdResult<HandleResponse> {
    store_exchange(deps, &exchange)?;

    Ok(HandleResponse::default())
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
