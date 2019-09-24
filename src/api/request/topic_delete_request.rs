use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopicDeleteRequest {
    pub id: String,
}