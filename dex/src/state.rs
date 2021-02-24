use std::any::type_name;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use secret_toolkit::serialization::{Bincode2, Serde};
use cosmwasm_std::{HumanAddr, ReadonlyStorage, StdError, StdResult, Storage};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub token_addr: ContractInfo,
    pub factory_addr: HumanAddr,
    pub name: String,
    pub symbol: String,
    pub decimals: u8
}

/// code hash and address of a contract
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct ContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: HumanAddr,
}

/// Returns StdResult<()> resulting from saving the config to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `config` - a reference to a Config struct
pub fn save_config(storage: &mut impl Storage, config: &Config) -> StdResult<()> {
    save(storage, CONFIG_KEY, config)
}

/// Returns StdResult<()> resulting from saving the config to storage
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item should go to
pub fn load_config(storage: &impl ReadonlyStorage) -> StdResult<Config> {
    load(storage, CONFIG_KEY)
}

fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
    Ok(())
}

fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}

fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Bincode2::deserialize(&value).map(Some),
        None => Ok(None),
    }
}
