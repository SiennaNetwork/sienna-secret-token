use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{HumanAddr, Uint128};
use amm_shared::{ContractInfo, TokenPair, TokenType, TokenPairAmount};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddLiquidity {
        input: TokenPairAmount
    },
    RemoveLiquidity {
    },
    /// Sent by the LP token contract so that we can record its address
    OnLpTokenInit
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsgResponse {
    AddLiquidity {
    },
    RemoveLiquidity{
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    PairInfo,
    FactoryInfo,
    Pool
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsgResponse {
    PairInfo(TokenPair),
    FactoryInfo(ContractInfo),
    Pool(TokenPairAmount)
}
