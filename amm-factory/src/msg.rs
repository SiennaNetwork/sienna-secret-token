use cosmwasm_std::{HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use amm_shared::{TokenPair};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    //TODO: IDs used for WasmMsg::Instantiate, but it doesn't say what they are about
    pub token_code_id: u64,
    pub pair_code_id: u64,
    pub token_code_hash: String,
    pub pair_code_hash: String,
}

/// Represents the address of an exchange and the pair that it manages
#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Debug, Clone)]
pub struct Exchange {
    /// The pair that the contract manages.
    pub pair: TokenPair,
    /// Address of the contract that manages the exchange.
    pub address: HumanAddr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// Instantiates an exchange pair contract
    CreatePair {
        pair: TokenPair
    },
    /// Used by a newly instantiated exchange contract to send its address
    /// for the factory to register.
    RegisterExchange {
        exchange: Exchange
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetExchangePair {
        exchange_addr: HumanAddr,
    },
    GetPairExchangeAddress {
        pair: TokenPair
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    GetExchangePair {
        pair: TokenPair
    },
    GetPairExchangeAddress {
        address: HumanAddr
    }
}
