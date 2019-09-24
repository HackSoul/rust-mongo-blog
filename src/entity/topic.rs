use std::time::SystemTime;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub name: String,
    pub create_date: SystemTime
}