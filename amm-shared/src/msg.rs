use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::TokenPair;
use crate::ContractInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairInitMsg {
    /// Asset infos
    pub pair: TokenPair,
    /// Token contract code id for initialization
    pub token_code_id: u64,
    pub token_code_hash: String,
    /// Used by the exchange contract to
    /// send us back its address on init
    pub factory_info: ContractInfo,
}
