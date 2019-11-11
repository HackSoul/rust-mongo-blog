use crate::dao::mongo_connector;
use crate::entity::article::Article;

use mongodb::coll::results::InsertOneResult;
use mongodb::coll::results::UpdateResult;
use mongodb::coll::results::DeleteResult;
use mongodb::{Bson, bson, doc, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::cursor::Cursor;
use mongodb::oid::ObjectId;

use chrono::offset::Utc;
use chrono::DateTime;
use crate::api::request::article_find_request::ArticleFindRequest;

pub fn find_article_list(page: u32, request: ArticleFindRequest) -> Cursor {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");
    let mut search_vec = vec![
        doc! { "$sort" : { "create_date" : 1}},
        doc! { "$skip" : page },
        doc! { "$limit" : 20 }
    ];
    if request.category != None {
        search_vec.push(doc! { "$match": { "category": request.category.unwrap()}});
    }
    if request.technology != None {
        search_vec.push(doc! { "$match": { "technology": request.technology.unwrap()}});
    }
    coll.aggregate(search_vec, None).unwrap()
    //coll.find(None, None).sort(doc! {"create_date": 1}).skip(page).limit(20).unwrap()
}

pub fn create_article(article: Article) -> InsertOneResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");

    let datetime: DateTime<Utc> = article.create_date.into();
    let mut tags_list:Vec<Bson> = vec![];
    for str in article.tags {
        tags_list.push(Bson::String(str));
    }

    let doc = doc! {
        "title": article.title,
        "category": article.category,
        "technology": article.technology,
        "create_date": format!("{}", datetime.format("%Y-%m-%d %T")),
        "tags": Bson::Array(tags_list),
        "view_count": article.view_count,
        "introduce": article.introduce
    };

    coll.insert_one(doc, None)
        .ok().expect("Failed to insert document.")
}

pub fn update_article(id: &String, title: &String, category: &String, technology: &String, tags: &Vec<String>, introduce: &String) -> UpdateResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");

    let id = Bson::ObjectId(ObjectId::with_string(id).unwrap());
    let mut tags_list:Vec<Bson> = vec![];
    for str in tags {
        tags_list.push(Bson::String(String::from(str)));
    }
    let doc = doc! {
        "$set": {
            "title": title,
            "category": category,
            "technology": technology,
            "tags": tags_list,
            "introduce": introduce
        }
    };

    coll.update_one(doc!{"_id": id}, doc, None).unwrap()
}

pub fn delete_article(id: &String) -> DeleteResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");
    let id = Bson::ObjectId(ObjectId::with_string(id).unwrap());
    coll.delete_many(doc!{"_id": id}, None).unwrap()
}