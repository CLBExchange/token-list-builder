use std::str::FromStr;

use anyhow::{format_err, Result};
use reqwest::{IntoUrl, Url};
use tokenlist::TokenList;

/// Token list resolver based off of a URL.
#[derive(Clone)]
pub struct URLTokenListResolver {
    /// URL of the token list.
    pub url: Url,
}

impl FromStr for URLTokenListResolver {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<URLTokenListResolver> {
        if str == "solana" {
            Self::from_url("https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json")
        } else {
            Self::from_url(str)
        }
    }
}

impl URLTokenListResolver {
    pub fn from_url<T: IntoUrl>(url: T) -> Result<URLTokenListResolver> {
        Ok(URLTokenListResolver {
            url: url.into_url()?,
        })
    }

    pub async fn resolve(&self) -> Result<TokenList> {
        let response = reqwest::get(self.url.clone()).await?;
        if !response.status().is_success() {
            return Err(format_err!("url {} not found", self.url));
        }
        let text = response.text().await?;
        let token_list: TokenList = serde_json::from_str(&text)?;
        Ok(token_list)
    }
}
