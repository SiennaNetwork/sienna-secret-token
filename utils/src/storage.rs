use std::any::type_name;
use serde::{Serialize, de::DeserializeOwned};
use bincode2::{serialize, deserialize};
use cosmwasm_std::{ReadonlyStorage, StdError, StdResult, Storage};

/// Returns StdResult<()> resulting from saving an item to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    let serialized = serialize(value);

    if serialized.is_err() {
        return Err(StdError::generic_err("Error serializing"));
    }

    storage.set(key, serialized.unwrap().as_slice());
    Ok(())
}

/// Removes an item from storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}

/// Returns StdResult<T> from retrieving the item with the specified key. Returns a
/// StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    let value = &storage
        .get(key)
        .ok_or_else(|| StdError::not_found(type_name::<T>()))?;

    let result = deserialize(value);

    if result.is_err() {
        return Err(StdError::generic_err("Error deserializing"));
    }

    Ok(result.unwrap())
}
/*
/// Returns StdResult<Option<T>> from retrieving the item with the specified key.
/// Returns Ok(None) if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Bincode2::deserialize(&value).map(Some),
        None => Ok(None),
    }
}
*/
