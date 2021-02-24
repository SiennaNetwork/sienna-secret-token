use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{HumanAddr};
use ethnum::U256;
use crate::state::ContractInfo;
use crate::u256::U256Def;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub token_addr: ContractInfo,
    pub symbol: String,
    pub name: String,
    pub decimals: u8
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddLiquidity {
        #[serde(with = "U256Def")]
        min_liquidity: U256,
        #[serde(with = "U256Def")]
        max_tokens: U256,
        deadline: u64 
    },
    RemoveLiquidity {
        #[serde(with = "U256Def")]
        min_liquidity: U256,
        #[serde(with = "U256Def")]
        min_eth: U256,
        #[serde(with = "U256Def")]
        min_tokens: U256,
        deadline: u64
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsgResponse {
    #[serde(with = "U256Def")]
    AddLiquidity { 
        initial_liquidity: U256 
    },
    RemoveLiquidity{
        #[serde(with = "U256Def")]
        eth_amount: U256,
        #[serde(with = "U256Def")]
        token_amount: U256
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenAddress,
    FactoryAddress,
    #[serde(with = "U256Def")]
    GetEthToTokenInputPrice { eth_sold: U256 },
    #[serde(with = "U256Def")]
    GetEthToTokenOutputPrice{ tokens_bought: U256 },
    #[serde(with = "U256Def")]
    GetTokenToEthInputPrice{ tokens_sold: U256 },
    #[serde(with = "U256Def")]
    GetTokenToEthOutputPrice{ eth_bought: U256 }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsgResponse {
    TokenAddress(HumanAddr),
    FactoryAddress(HumanAddr),
    #[serde(with = "U256Def")]
    GetEthToTokenInputPrice{ tokens_bought: U256 },
    #[serde(with = "U256Def")]
    GetEthToTokenOutputPrice{ eth_sold: U256 },
    #[serde(with = "U256Def")]
    GetTokenToEthInputPrice{ eth_bought: U256 },
    #[serde(with = "U256Def")]
    GetTokenToEthOutputPrice{ tokens_sold: U256 }
}
