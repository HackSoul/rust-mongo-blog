use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct ArticleDeleteResponse {
    pub deleted_count: i32
}