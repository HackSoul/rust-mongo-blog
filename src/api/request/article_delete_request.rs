use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArticleDeleteRequest {
    pub id: String,
}