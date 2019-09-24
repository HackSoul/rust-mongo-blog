use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct TopicUpdateResponse {
    pub modified_count: i32
}