use cosmwasm_std::{StdResult, Storage, HumanAddr};
use cosmwasm_storage::{ReadonlySingleton, Singleton};
use serde::{Serialize,Deserialize};
use schemars::JsonSchema;

use amm_shared::{TokenPair, ContractInfo, ContractInstantiationInfo};
use utils::storage::{load, save};

const CONFIG_KEY: &[u8] = b"config"; 

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub factory_info: ContractInfo,
    pub lp_token_contract: ContractInstantiationInfo,
    pub pair: TokenPair,
    /// The address of the current contract, because they decided that they don't want
    /// to give you an `Env` in the query callback.
    pub contract_addr: HumanAddr
}

pub fn store_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    save(storage, CONFIG_KEY, config)
}

pub fn load_config<S: Storage>(storage: &S) -> StdResult<Config> {
    load(storage, CONFIG_KEY)
}
