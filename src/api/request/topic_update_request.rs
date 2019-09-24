use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopicUpdateRequest {
    pub id: String,
    pub name: String
}