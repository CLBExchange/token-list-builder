use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TokenListMeta {
    pub id: String,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListMeta {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Lists {
    pub meta: ListMeta,
    pub lists: Vec<TokenListMeta>,
}
