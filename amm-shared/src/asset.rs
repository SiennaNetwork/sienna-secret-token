use std::fmt;

use cosmwasm_std::{
    Api, CanonicalAddr, Extern, HumanAddr, Querier, StdResult, Storage, Uint128
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::snip20::balance_query;

const BLOCK_SIZE: usize = 256;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenPairAmount {
    pub pair: TokenPair,
    pub amount_0: Uint128,
    pub amount_1: Uint128
}

impl fmt::Display for TokenPairAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token 1: {} {} \n Token 2: {} {}",
            self.pair.0, self.amount_0, self.pair.1, self.amount_1
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TokenPair(pub TokenType, pub TokenType);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    CustomToken {
        contract_addr: HumanAddr,
        token_code_hash: String,
        viewing_key: String,
    },
    NativeToken {
        denom: String,
    },
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::NativeToken { denom } => write!(f, "{}", denom),
            TokenType::CustomToken { contract_addr, .. } => write!(f, "{}", contract_addr),
        }
    }
}

impl TokenType {
    pub fn get_canonical_address<S: Storage, A: Api, Q: Querier>(
        &self, 
        deps: &Extern<S, A, Q>
    ) -> StdResult<Option<CanonicalAddr>> {
        match self {
            TokenType::NativeToken { .. } => Ok(None),
            TokenType::CustomToken { contract_addr, .. } => Ok(Some(deps.api.canonical_address(contract_addr)?)),
        }
    }

    pub fn is_native_token(&self) -> bool {
        match self {
            TokenType::NativeToken { .. } => true,
            TokenType::CustomToken { .. } => false,
        }
    }

    pub fn is_custom_token(&self) -> bool {
        match self {
            TokenType::NativeToken { .. } => false,
            TokenType::CustomToken { .. } => true,
        }
    }

    pub fn query_balance<S: Storage, A: Api, Q: Querier>(
        &self,
        deps: &Extern<S, A, Q>,
        exchange_addr: HumanAddr
    ) -> StdResult<Uint128> {
        match self {
            TokenType::NativeToken { denom } => {
                let result = deps.querier.query_balance(exchange_addr, denom)?;
                Ok(result.amount)
            },
            TokenType::CustomToken { contract_addr, token_code_hash, viewing_key } => {
                let result = balance_query(
                    &deps.querier,
                    exchange_addr.clone(),
                    viewing_key.clone(),
                    BLOCK_SIZE,
                    token_code_hash.clone(),
                    contract_addr.clone()
                )?;

                Ok(result.amount)
            }
        }
    }
}

impl TokenPair {
    pub fn query_balances<S: Storage, A: Api, Q: Querier>(
        &self,
        deps: &Extern<S, A, Q>,
        exchange_addr: HumanAddr
    ) -> StdResult<(Uint128, Uint128)> {
        let amount_0 = self.0.query_balance(deps, exchange_addr.clone())?;
        let amount_1 = self.1.query_balance(deps, exchange_addr)?;

        Ok((amount_0, amount_1))
    }
}

impl PartialEq for TokenPair {
    fn eq(&self, other: &TokenPair) -> bool {
        (self.0 == other.0 || self.0 == other.1) && (self.1 == other.0 || self.1 == other.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_pair_equality() {
        let pair = TokenPair(
            TokenType::CustomToken {
                contract_addr: "address".into(),
                token_code_hash: "hash".into(),
                viewing_key: "viewing_key".into(),
            },
            TokenType::NativeToken {
                denom: "denom".into()
            }
        );

        let pair2 = TokenPair(pair.1.clone(), pair.0.clone());

        assert_eq!(pair, pair.clone());
        assert_eq!(pair2, pair2.clone());
        assert_eq!(pair, pair2);
    }
}
