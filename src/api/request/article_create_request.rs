use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArticleCreateRequest {
    pub title: String,
    pub topic_name: String,
    pub topic_id: String,
    pub tags: Vec<String>,
    pub markdown: String
}