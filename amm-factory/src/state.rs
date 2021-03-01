use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CanonicalAddr, HumanAddr, Storage, Querier, Api, StdResult, Extern, ReadonlyStorage, StdError};
use cosmwasm_storage::{Bucket};
use utils::storage::{save, load};
use amm_shared::{TokenPair, TokenType};

use crate::msg::{Exchange};

pub static CONFIG_KEY: &[u8] = b"config";
static PREFIX_PAIR_INFO: &[u8] = b"pair_info";

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub token_code_id: u64,
    pub pair_code_id: u64,
    pub token_code_hash: String,
    pub pair_code_hash: String,
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

/// Returns StdResult<Config> resulting from retrieving the config from storage
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item should go to
pub fn load_config(storage: &impl ReadonlyStorage) -> StdResult<Config> {
    load(storage, CONFIG_KEY)
}

/// Returns StdResult<bool> indicating whether a pair has been created before or not.
/// Note that TokenPair(A, B) and TokenPair(B, A) is considered to be same.
pub fn try_store_pair<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    pair: &TokenPair
) -> StdResult<bool> {
    let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());

    let key = generate_pair_key(pair, &addr_first, &addr_second);

    let mut bucket: Bucket<S, TokenPair> = Bucket::new(PREFIX_PAIR_INFO, &mut deps.storage);

    let exists = bucket.may_load(&key)?;

    if exists.is_some() {
        return Ok(false);
    }

    bucket.save(&key, pair)?;

    Ok(true)
}

/// Stores information about an exchange contract. Returns an `StdError` if the exchange
/// already exists or if something else goes wrong.
pub fn store_exchange<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    exchange: &Exchange
) -> StdResult<()> {
    let Exchange {
        pair,
        address
    } = exchange;

    let canonical = deps.api.canonical_address(&address)?;
    let result = load(&deps.storage, canonical.as_slice());

    let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());

    let key = generate_pair_key(&pair, &addr_first, &addr_second);

    match result {
        Ok(_value) => Err(StdError::generic_err("Exchange address already exists")),
        Err(ref err) => match err {
            StdError::NotFound { .. } => { 
                save(&mut deps.storage, canonical.as_slice(), &pair)?;
                save(&mut deps.storage, &key, &canonical)
            },
            _ => result
        }
    }
}

pub fn get_pair_for_address<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    exchange_addr: &HumanAddr
) -> StdResult<TokenPair> {
    let canonical = deps.api.canonical_address(exchange_addr)?;

    load(&deps.storage, canonical.as_slice())
}

pub fn get_address_for_pair<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    pair: &TokenPair
) -> StdResult<HumanAddr> {
    let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());

    let key = generate_pair_key(&pair, &addr_first, &addr_second);

    load(&deps.storage, &key)
}

fn generate_pair_key<'a>(
    pair: &'a TokenPair,
    addr_first: &'a CanonicalAddr,
    addr_second: &'a CanonicalAddr
) -> Vec<u8> {
    let mut bytes: Vec<&[u8]> = Vec::new();

    match &pair.0 {
        TokenType::NativeToken { denom } => bytes.push(denom.as_bytes()),
        TokenType::CustomToken { .. } => bytes.push(addr_first.as_slice())
    }

    match &pair.1 {
        TokenType::NativeToken { denom } => bytes.push(denom.as_bytes()),
        TokenType::CustomToken { .. } => bytes.push(addr_second.as_slice())
    }

    bytes.sort_by(|a, b| a.cmp(&b));

    bytes.concat()
}
