use mongodb::bson::{doc, oid::ObjectId, DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyCollection {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author_id: ObjectId,
    pub movie_id: i64,
    pub movie_title: String,
    pub genre: Option<Vec<super::movie::Genre>>,
    pub image_url: String,
    pub rating: Option<f64>,
    pub content: Option<String>,
    pub is_post: bool,
    pub me_too: u64,
    pub watched_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: DateTime,
}

impl Default for MyCollection {
    fn default() -> Self {
        Self {
            id: None,
            author_id: ObjectId::new(),
            movie_id: 0,
            movie_title: "".to_string(),
            genre: None,
            image_url: "".to_string(),
            rating: None,
            content: None,
            is_post: false,
            me_too: 0,
            watched_at: None,
            created_at: Some(DateTime::now()),
            updated_at: DateTime::now(),
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
                "updated_at": {
                    "bsonType": "date",
                    "description": "must be a date"
                },
            }
         }
    }
}
