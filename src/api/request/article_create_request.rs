use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArticleCreateRequest {
    pub title: String, //题目
    pub category: String, //分类，如大数据，云原声等
    pub technology: String, //技术，如Docker，k8s等
    pub tags: Vec<String>, //标签
    pub introduce: String //短介绍
}