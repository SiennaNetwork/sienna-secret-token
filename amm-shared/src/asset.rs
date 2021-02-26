use std::fmt;

use cosmwasm_std::{
    Api, CanonicalAddr, Extern, HumanAddr, Querier, StdResult, Storage, Uint128
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Asset {
    pub token_type: TokenType,
    pub amount: Uint128,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.amount, self.token_type)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
        deps: &Extern<S, A, Q>) -> StdResult<Option<CanonicalAddr>> {
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
}
