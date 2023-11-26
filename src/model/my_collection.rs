use bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyCollection {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author_id: Option<ObjectId>,
    pub movie_id: Option<i64>,
    pub movie_title: Option<String>,
    pub genre: Option<Vec<super::movie::Genre>>,
    pub image_url: String,
    pub rating: Option<f64>,
    pub content: Option<String>,
    pub is_post: Option<bool>,
    pub me_too: Option<u64>,
    pub watched_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
}

impl Default for MyCollection {
    fn default() -> Self {
        Self {
            id: Some(ObjectId::new()),
            author_id: Some(ObjectId::new()),
            movie_id: Some(0),
            movie_title: Some("".to_string()),
            genre: None,
            image_url: "".to_string(),
            rating: Some(0.),
            content: Some("".to_string()),
            is_post: Some(false),
            me_too: Some(0),
            watched_at: Some(DateTime::now()),
            created_at: Some(DateTime::now()),
        }
    }
}

pub fn collection_schema() -> Document {
    doc! {
        "$jsonSchema": doc! {
            "bsonType": "object",
            "title": "Collection Object Validation",
            "required": ["author_id", "movie_id", "movie_title", "image_url", "is_post"],
            "properties": doc! {
                "author_id": {
                    "bsonType": "objectId",
                    "description": "must be a objectId and is required"
                },
                "movie_id": {
                    "bsonType": "int",
                    "description": "must be a int and is required"
                },
                "movie_title": {
                    "bsonType": "string",
                    "description": "must be a string and is required"
                },
                "genre": {
                    "bsonType": "array",
                    "description": "must be a array",
                    "items": {
                        "bsonType": "object",
                        "required": ["id", "name"],
                        "properties": {
                            "id": {
                                "bsonType": "int",
                                "description": "must be a int and is required"
                            },
                            "name": {
                                "bsonType": "string",
                                "description": "must be a string and is required"
                            }
                        }
                    }
                },
                "image_url": {
                    "bsonType": "string",
                    "description": "must be a string and is required"
                },
                "rating": {
                    "bsonType": "int",
                    "description": "must be a int",
                    "minimum": 0,
                    "maximum": 5
                },
                "content": {
                    "bsonType": "string",
                    "description": "must be a string"
                },
                "is_post": {
                    "bsonType": "bool",
                    "description": "must be a bool and is required"
                },
                "me_too": {
                    "bsonType": "int",
                    "description": "must be a int",
                    "minimum": 0
                },
                "watched_at": {
                    "bsonType": "date",
                    "description": "must be a date"
                },
                "created_at": {
                    "bsonType": "date",
                    "description": "must be a date"
                },
            }
         }
    }
}
