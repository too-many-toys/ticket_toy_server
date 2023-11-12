use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub movie_id: i64,
    pub movie_title: String,
    pub title: String,
    pub content: String,
    pub cinema: Option<String>,
    pub author_id: Option<ObjectId>,
    pub author_nickname: Option<String>,
    pub image_url: Option<String>,
    pub rating: Option<f64>,
    pub genre: Vec<super::movie::Genre>,
    pub adult: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
