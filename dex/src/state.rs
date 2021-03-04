use cosmwasm_std::{StdResult, Storage};
use cosmwasm_storage::{ReadonlySingleton, Singleton};
use serde::{Serialize,Deserialize};
use schemars::JsonSchema;

use amm_shared::{TokenPair, ContractInfo, ContractInstantiationInfo};
use utils::storage::{load, save};

const PAIR_INFO_KEY: &[u8] = b"pair_info";
const CONFIG_KEY: &[u8] = b"config"; 

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub factory_info: ContractInfo,
    pub lp_token_contract: ContractInstantiationInfo,
}

pub fn store_pair<S: Storage>(storage: &mut S, pair: &TokenPair) -> StdResult<()> {
    save(storage, PAIR_INFO_KEY, pair)
}

pub fn load_pair<S: Storage>(storage: &S) -> StdResult<TokenPair> {
    load(storage, PAIR_INFO_KEY)
}

pub fn store_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    save(storage, CONFIG_KEY, config)
}

pub fn load_config<S: Storage>(storage: &S) -> StdResult<Config> {
    load(storage, CONFIG_KEY)
}
