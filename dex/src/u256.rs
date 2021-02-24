use schemars::{JsonSchema};
use serde::{Serialize, Deserialize};
use ethnum::U256;

// Need to use Newtype pattern because of orphan rule:
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types 
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, JsonSchema)]
#[repr(transparent)]
#[serde(remote = "U256")]
#[derive(Serialize, Deserialize)]
pub struct U256Def(pub [u128; 2]);

//Yes, its annoying to use #[serde(with = "U256Def")] anytime a U256 is included
//as a struct/enum member, but its the only way...
