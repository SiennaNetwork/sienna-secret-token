use cosmwasm_std::{HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Code hash and address of a contract.
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct ContractInfo {
    pub code_hash: String,
    pub address: HumanAddr,
}

/// Info used to instantiate a contract
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct ContractInstantiationInfo {
    pub code_hash: String,
    pub id: u64
}

impl Default for ContractInfo {
    fn default() -> Self {
        ContractInfo {
            code_hash: "".into(),
            address: HumanAddr::default()
        }
    }
}
