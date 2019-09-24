use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct TopicDeleteResponse {
    pub deleted_count: i32
}