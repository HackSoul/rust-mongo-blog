use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct ArticleUpdateResponse {
    pub modified_count: i32
}