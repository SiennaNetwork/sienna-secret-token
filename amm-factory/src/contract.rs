use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, WasmMsg, CosmosMsg, log
};
use amm_shared::{TokenPair, PairInitMsg};

use crate::msg::{InitMsg, HandleMsg, QueryMsg};
use crate::state::{save_config, load_config, Config, try_store_pair};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let InitMsg {
        token_code_id,
        pair_code_id,
        token_code_hash,
        pair_code_hash,
    } = msg;

    let config = Config {
        token_code_id,
        pair_code_id,
        token_code_hash,
        pair_code_hash,
    };

    save_config(&mut deps.storage, &config);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreatePair { pair } => try_create_pair(deps, &env, pair),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    unimplemented!();
}

fn try_create_pair<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    pair: TokenPair
) -> StdResult<HandleResponse> {
    if try_store_pair(deps, &pair)? == false {
        return Err(StdError::generic_err("Pair already exists"));
    }

    let config = load_config(&deps.storage)?;

    Ok(HandleResponse{
        messages: vec![
            CosmosMsg::Wasm(
                WasmMsg::Instantiate {
                    code_id: config.pair_code_id,
                    send: vec![],
                    label: format!(
                        "{}-{}-pair-{}-{}",
                        pair.0,
                        pair.1,
                        env.contract.address.clone(),
                        config.pair_code_id
                    ),
                    msg: to_binary(
                        &PairInitMsg {
                            pair,
                            token_code_id: config.token_code_id,
                            token_code_hash: config.token_code_hash
                        }
                    )?
                }
            )
        ],
        log: vec![
            log("action", "create_pair"),
            log("pair", format!("{}-{}", pair.0, pair.1)),
        ],
        data: None
    })
}
