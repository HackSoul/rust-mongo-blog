use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArticleUpdateRequest {
    pub id: String,
    pub title: String,
    pub category: String,
    pub technology: String,
    pub tags: Vec<String>
}