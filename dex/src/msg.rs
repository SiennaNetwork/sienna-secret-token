use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{HumanAddr, Uint128};
use amm_shared::{ContractInfo, TokenPair, TokenType, TokenPairAmount};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddLiquidity {
        input: LiquidityDeposit
    },
    RemoveLiquidity {
    }
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

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LiquidityDeposit {
    /// This is only used for validation. Since the factory should have
    /// provided the correct exchange contract for the requested pair.
    /// Its seems like it is more secure to check again though.
    pub pair: TokenPair,
    pub amount_0: Uint128,
    pub amount_1: Uint128
}
