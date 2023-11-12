use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author_id: Option<ObjectId>,
    pub author_age: Option<super::user::UserAge>,
    pub movie_id: i64,
    pub movie_title: String,
    pub genre: Vec<super::movie::Genre>,
    pub image_url: Option<String>,
    pub rating: Option<f64>,
    pub movie_detail: Option<super::movie::Movie>,
    pub created_at: i64,
    pub updated_at: i64,
}
