use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopicCreateRequest {
    pub name: String,
}