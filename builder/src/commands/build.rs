use chrono::prelude::{DateTime, Utc};
use reqwest::Url;
use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};

use anyhow::{format_err, Result};
use futures::StreamExt;

use crate::lists::Lists;
use tokenlist::{TagDetails, TokenInfo, TokenList};

pub async fn handler(lists: &Lists) -> Result<()> {
    let mut tokens: BTreeMap<(u32, String), TokenInfo> = Default::default();
    let mut chains: HashSet<u32> = HashSet::new();
    let mut tags: BTreeMap<String, TagDetails> = Default::default();

    let mut lists_iter = futures::stream::iter(lists.lists.clone().into_iter());
    while let Some(meta) = lists_iter.next().await {
        let list_path = PathBuf::from(format!("lists/{}.json", meta.id));
        let list_str = std::fs::read_to_string(list_path)?;
        let token_list: TokenList = serde_json::from_str(&list_str)
            .map_err(|e| format_err!("Error parsing {}: {}", &meta.id, e))?;

        token_list.tags.into_iter().for_each(|(tag, tag_details)| {
            tags.insert(tag, tag_details);
        });

        token_list.tokens.iter().for_each(|token| {
            chains.insert(token.chain_id);
            tokens
                .entry((token.chain_id, token.address.to_string()))
                .or_insert_with(|| token.clone());
        });
    }

    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();

    let mut sorted_tokens: Vec<TokenInfo> = tokens.values().cloned().collect();
    sorted_tokens.sort();
    let final_list: TokenList = TokenList {
        name: lists.meta.name.clone(),
        tokens: sorted_tokens,
        timestamp: now,
        tags,
        logo_uri: Url::from_str("https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png")?,
    };

    std::fs::create_dir_all("out/")?;
    let out = PathBuf::from("out/token-list.json".to_string());
    std::fs::write(out, serde_json::to_string(&final_list)?)?;

    let out_simple = PathBuf::from("out/token-list.simple.json".to_string());

    let mut simple_list = final_list.clone();
    simple_list.name = format!("{} (Simple)", simple_list.name);
    simple_list.simplify();
    std::fs::write(out_simple, serde_json::to_string(&simple_list)?)?;

    for chain_id in chains.iter() {
        std::fs::create_dir_all(format!("out/{}/", chain_id))?;
        for token in tokens.values().filter(|t| t.chain_id == *chain_id) {
            let out = PathBuf::from(format!("out/{}/{}.json", chain_id, token.address));
            std::fs::write(out, serde_json::to_string(&token)?)?;
        }

        std::fs::write(
            format!("out/{}/_certified-token-list.json", chain_id),
            serde_json::to_string(&final_list.clone().filter_chain(*chain_id))?,
        )?;
        std::fs::write(
            format!("out/{}/_certified-token-list.simple.json", chain_id),
            serde_json::to_string(&simple_list.clone().filter_chain(*chain_id))?,
        )?;
    }

    Ok(())
}
