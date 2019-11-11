use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArticleFindRequest {
    pub category: Option<String>,
    pub technology: Option<String>
}