use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::HumanAddr;

use crate::TokenPair;
use crate::{ContractInfo, ContractInstantiationInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairInitMsg {
    /// Asset infos
    pub pair: TokenPair,
    /// LP token instantiation info
    pub lp_token_contract: ContractInstantiationInfo,
    /// Used by the exchange contract to
    /// send us back its address on init
    pub factory_info: ContractInfo,
}

/// TokenContract InitMsg
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LpTokenInitMsg {
    pub name: String,
    pub admin: HumanAddr,
    pub symbol: String,
    pub decimals: u8,
}
