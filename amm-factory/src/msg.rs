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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// Instantiates an exchange pair contract
    CreatePair {
        pair: TokenPair
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}
