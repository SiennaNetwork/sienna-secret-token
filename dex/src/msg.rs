use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{HumanAddr, Uint128};
use amm_shared::{ContractInfo, TokenPair};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddLiquidity {
        min_liquidity: Uint128,
        max_tokens: Uint128,
        deadline: u64 
    },
    RemoveLiquidity {
        min_liquidity: Uint128,
        min_eth: Uint128,
        min_tokens: Uint128,
        deadline: u64
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsgResponse {
    AddLiquidity { 
        initial_liquidity: Uint128 
    },
    RemoveLiquidity{
        eth_amount: Uint128,
        token_amount: Uint128
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    PairInfo,
    FactoryInfo,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsgResponse {
    PairInfo(TokenPair),
    FactoryInfo(ContractInfo),
}
