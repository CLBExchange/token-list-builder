//! Serde Solana Pubkey serializer/deserializer.
//!
//! # Installation
//!
//! Add `pubkey` to your `Cargo.toml`.
//!
//! # Usage
//!
//! ```
//! use solana_program::pubkey::Pubkey;
//!
//! #[derive(serde::Serialize, serde::Deserialize)]
//! pub struct MyStruct {
//!     /// Token pubkey.
//!     #[serde(with = "pubkey")]
//!     pub address: Pubkey,
//! }
//! ```
//!
//! # License
//!
//! The `pubkey` crate is licensed under the Apache 2.0 License.

use serde::{self, Deserialize, Deserializer, Serializer};
pub use solana_program::pubkey::Pubkey;
use std::str::FromStr;

/// The signature of a serialize_with function must follow the pattern:
///
///    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
///    where
///        S: Serializer
///
/// although it may also be generic over the input types T.
pub fn serialize<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", pubkey);
    serializer.serialize_str(&s)
}

/// The signature of a deserialize_with function must follow the pattern:
///
///    fn deserialize<'de, D>(D) -> Result<T, D::Error>
///    where
///        D: Deserializer<'de>
///
/// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}
