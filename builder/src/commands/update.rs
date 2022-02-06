use anyhow::{format_err, Result};
use futures::{StreamExt, TryFutureExt};
use std::str::FromStr;

use crate::{
    lists::{Lists, TokenListMeta},
    resolvers::URLTokenListResolver,
};

async fn fetch_token_list(meta: TokenListMeta) -> Result<()> {
    println!("Fetching {} from {}", meta.id, meta.url);
    let resolver = URLTokenListResolver::from_str(&meta.url)?;
    let mut list = resolver.resolve().await?;
    list.tokens.sort();
    println!("Fetched {} with {} tokens", meta.id, list.tokens.len());
    std::fs::write(
        format!("lists/{}.json", meta.id),
        serde_json::to_string_pretty(&list)?,
    )?;
    Ok(())
}

pub async fn handler(lists: &Lists) -> Result<()> {
    std::fs::create_dir_all("lists/")?;

    let index = lists
        .lists
        .iter()
        .map(|list| list.id.clone())
        .collect::<Vec<_>>();
    std::fs::write("lists/_index.json", serde_json::to_string_pretty(&index)?)?;

    let fetches = futures::stream::iter(lists.lists.clone().into_iter().map(|list| {
        let list_id = list.id.clone();
        tokio::spawn(
            fetch_token_list(list)
                .map_err(move |err| format_err!("Error parsing list {}: {}", &list_id, err)),
        )
    }))
    .buffer_unordered(3)
    .collect::<Vec<_>>()
    .await;

    // await all results
    for res in fetches {
        match res? {
            Ok(_) => (),
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
    Ok(())
}
