use chrono::prelude::{DateTime, Utc};
use reqwest::Url;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};

use anyhow::Result;
use futures::StreamExt;

use crate::lists::Lists;
use tokenlist::{TokenInfo, TokenList};

pub async fn handler(lists: &Lists) -> Result<()> {
    let mut tokens: HashMap<(u32, String), TokenInfo> = Default::default();
    let mut chains: HashSet<u32> = HashSet::new();

    let mut lists_iter = futures::stream::iter(lists.lists.clone().into_iter());
    while let Some(meta) = lists_iter.next().await {
        let list_path = PathBuf::from(format!("lists/{}.json", meta.id));
        let list_str = std::fs::read_to_string(list_path)?;
        let token_list: TokenList = serde_json::from_str(&list_str)?;

        token_list.tokens.iter().for_each(|token| {
            chains.insert(token.chain_id);
            tokens
                .entry((token.chain_id, token.address.to_string()))
                .or_insert_with(|| token.clone());
        });
    }

    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();

    let final_list: TokenList = TokenList {
        name: lists.meta.name.clone(),
        tokens: tokens.values().cloned().collect(),
        timestamp: now,
        tags: Default::default(),
        logo_uri: Url::from_str("https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png")?,
    };

    std::fs::create_dir_all("out/")?;
    let out = PathBuf::from("out/token-list.json".to_string());
    std::fs::write(out, serde_json::to_string(&final_list)?)?;

    for chain_id in chains.iter() {
        std::fs::create_dir_all(format!("out/{}/", chain_id))?;
        for token in tokens.values().filter(|t| t.chain_id == *chain_id) {
            let out = PathBuf::from(format!("out/{}/{}.json", chain_id, token.address));
            std::fs::write(out, serde_json::to_string(&token)?)?;
        }
    }

    Ok(())
}
