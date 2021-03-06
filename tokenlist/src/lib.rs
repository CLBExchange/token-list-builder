//! Parses token lists according to the token list standard.
//!
//! # License
//!
//! The `tokenlist` crate is licensed under the Apache 2.0 License.
#![deny(missing_docs)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use solana_program::pubkey::Pubkey;
use std::{collections::BTreeMap, str::FromStr};
use url::Url;

fn ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'a> + Default,
    D: Deserializer<'a>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

fn default_token_list_logo() -> Url {
    Url::from_str("https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png").unwrap()
}

/// ID of a Solana chain.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum ChainID {
    /// Solana mainnet.
    MainnetBeta = 101,
    /// Solana testnet.
    Testnet = 102,
    /// Solana devnet.
    Devnet = 103,
}

/// Extra information about a token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenExtensions {
    /// Website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The bridge contract.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bridgeContract")]
    pub bridge_contract: Option<String>,
    /// The asset contract.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetContract")]
    pub asset_contract: Option<String>,
    /// Explorer link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer: Option<String>,
    /// Twitter link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    /// GitHub link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    /// Medium link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// Telegram announcement link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgann: Option<String>,
    /// Telegram group link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tggroup: Option<String>,
    /// Discord link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    /// Serum V3 USDT market.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serumV3Usdt")]
    pub serum_v3_usdt: Option<String>,
    /// Serum V3 USDC market.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serumV3Usdc")]
    pub serum_v3_usdc: Option<String>,
    /// Coingecko API ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coingeckoId")]
    pub coingecko_id: Option<String>,
    /// URL of the image representing this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    /// Brief description of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Underlying tokens backing this token.
    /// For example: if this is a Uniswap LP token, this would be the two tokens.
    #[serde(skip_serializing_if = "Option::is_none", rename = "underlyingTokens")]
    pub underlying_tokens: Option<Vec<String>>,

    /// The protocol or app that this token originates from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Unknown extensions.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Token information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Name of the token.
    pub name: String,
    /// Symbol of the token.
    pub symbol: String,
    /// Logo of the token. Highly recommended.
    /// If the provided logo is invalid, this value is discarded.
    #[serde(
        deserialize_with = "ok_or_default",
        default = "Option::default",
        skip_serializing_if = "Option::is_none",
        rename = "logoURI"
    )]
    pub logo_uri: Option<Url>,
    /// Number of decimals of the token.
    pub decimals: u8,
    /// Token pubkey.
    #[serde(with = "pubkey")]
    pub address: Pubkey,
    /// Chain ID of the token.
    #[serde(rename = "chainId")]
    pub chain_id: u32,
    /// Tags of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Token extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<TokenExtensions>,
}

impl Eq for TokenInfo {}

impl PartialEq for TokenInfo {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address && self.chain_id == other.chain_id
    }
}

impl PartialOrd for TokenInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TokenInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.chain_id == other.chain_id {
            self.address.to_string().cmp(&other.address.to_string())
        } else {
            self.chain_id.cmp(&other.chain_id)
        }
    }
}

impl TokenInfo {
    /// Removes the tags and extensions from the [TokenInfo].
    /// This is useful for making smaller token lists.
    pub fn simplify(&mut self) {
        self.tags = None;
        self.extensions = None;
    }
}

/// Details about what a tag is.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagDetails {
    /// Name of the tag.
    pub name: String,
    /// Description of what the tag is.
    pub description: String,
}

/// A list of SPL tokens.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenList {
    /// Name of the token list.
    pub name: String,
    /// Logo URI of the token list.
    #[serde(default = "default_token_list_logo", rename = "logoURI")]
    pub logo_uri: Url,
    /// All tags that may be referenced in the token list.
    pub tags: BTreeMap<String, TagDetails>,
    /// When the token list was last updated.
    pub timestamp: DateTime<Utc>,
    /// The tokens in the token list.
    pub tokens: Vec<TokenInfo>,
}

impl TokenList {
    /// Filters the tokens in the token list by the given chain ID.
    pub fn filter_chain(&self, chain_id: u32) -> TokenList {
        TokenList {
            tokens: self
                .tokens
                .clone()
                .into_iter()
                .filter(|t| t.chain_id == chain_id)
                .collect(),
            ..self.clone()
        }
    }

    /// Strips extraneous metadata from the token list.
    pub fn simplify(&mut self) {
        self.tags = BTreeMap::new();
        self.tokens = self
            .tokens
            .iter()
            .map(|token| {
                let mut token = token.clone();
                token.simplify();
                token
            })
            .collect();
    }
}
