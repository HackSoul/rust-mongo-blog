use std::time::SystemTime;

pub struct Article {
    pub title: String,
    pub topic_id: String,
    pub topic_name: String,
    pub create_date: SystemTime,
    pub tags: Vec<String>,
    pub view_count: i32,
    pub markdown: String,
}