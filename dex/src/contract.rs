use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, ReadonlyStorage, QueryResult
};
use secret_toolkit::snip20::{register_receive_msg};
use ethnum::U256;

use crate::msg::{HandleMsg, HandleMsgResponse, InitMsg, QueryMsg, QueryMsgResponse};
use crate::state::{Config, save_config, load_config};

/// Pad handle responses and log attributes to blocks
/// of 256 bytes to prevent leaking info based on response size
pub const BLOCK_SIZE: usize = 256;

//implements: https://github.com/Uniswap/uniswap-v1/blob/c10c08d81d6114f694baa8bd32f555a40f6264da/contracts/uniswap_exchange.vy#L32
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let config = Config {
        token_addr: msg.token_addr.clone(),
        factory_addr: env.message.sender,
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals
    };
    
    save_config(&mut deps.storage, &config)?;

    Ok(InitResponse {
        messages: vec![
            register_receive_msg(
                env.contract_code_hash,
                None,
                BLOCK_SIZE, 
                msg.token_addr.code_hash, 
                msg.token_addr.address
            )?
        ],
        log: vec![]
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMsg,
) -> StdResult<HandleResponse> {
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> QueryResult {
    let config = load_config(&deps.storage)?;

    match msg {
        QueryMsg::TokenAddress => to_binary(&QueryMsgResponse::TokenAddress(config.token_addr.address)),
        QueryMsg::FactoryAddress => to_binary(&QueryMsgResponse::FactoryAddress(config.factory_addr)),
        QueryMsg::GetEthToTokenInputPrice { eth_sold } => get_eth_to_token_input_price(&deps.storage, eth_sold),
        _ => unimplemented!()
    }
}

//implements: https://github.com/Uniswap/uniswap-v1/blob/c10c08d81d6114f694baa8bd32f555a40f6264da/contracts/uniswap_exchange.vy#L416
fn get_eth_to_token_input_price(storage: &impl ReadonlyStorage, eth_sold: U256) -> QueryResult {
    if(eth_sold == 0) {
        return Err(StdError::generic_err("Amount must larger than zero."));
    }

    unimplemented!();
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

}
*/
