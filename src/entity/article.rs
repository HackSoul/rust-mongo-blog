use std::time::SystemTime;

pub struct Article {
    pub title: String, //题目
    pub category: String, //分类，如大数据，云原声等
    pub technology: String, //技术，如Docker，k8s等
    pub create_date: SystemTime, //创建时间
    pub tags: Vec<String>, //标签
    pub view_count: i32, //访问次数
}