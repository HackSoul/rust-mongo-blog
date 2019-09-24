use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArticleUpdateRequest {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub markdown: String,
}