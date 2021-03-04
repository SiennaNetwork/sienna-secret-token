use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CanonicalAddr, HumanAddr, Storage, Querier, Api, StdResult, Extern, ReadonlyStorage, StdError};
use utils::storage::{save, load};
use amm_shared::{TokenPair, TokenType, ContractInstantiationInfo};

use crate::msg::{Exchange};

const CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub lp_token_contract: ContractInstantiationInfo,
    pub pair_contract: ContractInstantiationInfo,
    pub token_addr: HumanAddr,
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
pub fn pair_exists<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    pair: &TokenPair
) -> StdResult<bool> {
    let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());

    let key = generate_pair_key(pair, &addr_first, &addr_second);

    if let Some(_) = deps.storage.get(&key) {
        return Ok(true);
    }

    Ok(false)
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
    
    if let Some(_) = deps.storage.get(canonical.as_slice()) {
        return Err(StdError::generic_err("Exchange address already exists"));
    }

    let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());

    let key = generate_pair_key(&pair, &addr_first, &addr_second);

    if let Some(_) = deps.storage.get(&key) {
        return Err(StdError::generic_err("Exchange address already exists"));
    }

    save(&mut deps.storage, canonical.as_slice(), &pair)?;
    save(&mut deps.storage, &key, &canonical)?;

    Ok(())
}

/// Get the exchange pair that the given contract address manages.
pub fn get_pair_for_address<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    exchange_addr: &HumanAddr
) -> StdResult<TokenPair> {
    let canonical = deps.api.canonical_address(exchange_addr)?;

    load(&deps.storage, canonical.as_slice())
}

/// Get the address of an exchange contract which manages the given pair.
pub fn get_address_for_pair<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    pair: &TokenPair
) -> StdResult<CanonicalAddr> {
    let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());

    let key = generate_pair_key(&pair, &addr_first, &addr_second);

    load(&deps.storage, &key)
}

fn generate_pair_key(
    pair: &TokenPair,
    addr_first: &CanonicalAddr,
    addr_second: &CanonicalAddr
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies};
    use cosmwasm_std::{HumanAddr, Storage};

    fn create_deps() -> Extern<impl Storage, impl Api, impl Querier> {
        mock_dependencies(10, &[])
    }

    #[test]
    fn generates_the_same_key_for_swapped_pairs() -> StdResult<()> {
        fn cmp_pair<S: Storage, A: Api, Q: Querier>(
            deps: &Extern<S, A, Q>,
            pair: TokenPair
        ) -> StdResult<()> {
            let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
            let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    
            let key = generate_pair_key(&pair, &addr_first, &addr_second);
    
            let pair = swap_pair(&pair);
    
            let addr_first = pair.0.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
            let addr_second = pair.1.get_canonical_address(deps)?.unwrap_or_else(|| CanonicalAddr::default());
    
            let swapped_key = generate_pair_key(&pair, &addr_first, &addr_second);
    
            assert_eq!(key, swapped_key);

            Ok(())
        }

        fn swap_pair(pair: &TokenPair) -> TokenPair {
            TokenPair(
                pair.1.clone(),
                pair.0.clone()
            )
        }

        let ref deps = create_deps();

        cmp_pair(
            deps,
            TokenPair(
                TokenType::CustomToken {
                    contract_addr: HumanAddr("first_addr".into()),
                    token_code_hash: "13123adasd".into(),
                    viewing_key: "viewing_key1".into()
                },
                TokenType::CustomToken {
                    contract_addr: HumanAddr("scnd_addr".into()),
                    token_code_hash: "4534qwerqqw".into(),
                    viewing_key: "viewing_key2".into()
                }
            )
        )?;

        cmp_pair(
            deps,
            TokenPair(
                TokenType::NativeToken {
                    denom: "test1".into()
                },
                TokenType::NativeToken {
                    denom: "test2".into()
                },
            )
        )?;

        cmp_pair(
            deps,
            TokenPair(
                TokenType::NativeToken {
                    denom: "test3".into()
                },
                TokenType::CustomToken {
                    contract_addr: HumanAddr("third_addr".into()),
                    token_code_hash: "asd21312asd".into(),
                    viewing_key: "viewing_key3".into()
                }
            )
        )?;

        Ok(())
    }

    #[test]
    fn query_correct_exchange_info() -> StdResult<()> {
        let mut deps = create_deps();

        let pair = TokenPair (
            TokenType::CustomToken {
                contract_addr: HumanAddr("first_addr".into()),
                token_code_hash: "13123adasd".into(),
                viewing_key: "viewing_key1".into()
            },
            TokenType::CustomToken {
                contract_addr: HumanAddr("scnd_addr".into()),
                token_code_hash: "4534qwerqqw".into(),
                viewing_key: "viewing_key2".into()
            }  
        );

        let address = HumanAddr("ctrct_addr".into());

        let exchange = Exchange {
            pair: pair.clone(),
            address: address.clone()
        };

        store_exchange(&mut deps, &exchange)?;

        let retrieved_pair = get_pair_for_address(&deps, &exchange.address)?;
        let retrieved_address = get_address_for_pair(&deps, &pair)?;
        let retrieved_address = deps.api.human_address(&retrieved_address)?;
        
        assert_eq!(pair, retrieved_pair);
        assert_eq!(address, retrieved_address);

        Ok(())
    }

    #[test]
    fn only_one_exchange_per_factory() -> StdResult<()> {
        let ref mut deps = create_deps();

        let exchange = Exchange {
            pair: TokenPair (
                TokenType::CustomToken {
                    contract_addr: HumanAddr("first_addr".into()),
                    token_code_hash: "13123adasd".into(),
                    viewing_key: "viewing_key1".into()
                },
                TokenType::CustomToken {
                    contract_addr: HumanAddr("scnd_addr".into()),
                    token_code_hash: "4534qwerqqw".into(),
                    viewing_key: "viewing_key2".into()
                }  
            ),
            address: HumanAddr("ctrct_addr".into())
        };

        store_exchange(deps, &exchange)?;

        let exchange = Exchange {
            pair: TokenPair (
                TokenType::CustomToken {
                    contract_addr: HumanAddr("scnd_addr".into()),
                    token_code_hash: "4534qwerqqw".into(),
                    viewing_key: "viewing_key2".into()
                },
                TokenType::CustomToken {
                    contract_addr: HumanAddr("first_addr".into()),
                    token_code_hash: "13123adasd".into(),
                    viewing_key: "viewing_key1".into()
                },
            ),
            address: HumanAddr("other_addr".into())
        };
        
        match store_exchange(deps, &exchange) {
            Ok(_) => Err(StdError::generic_err("Exchange already exists")),
            Err(_) => Ok(())
        }
    }
}
