use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MeToo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub too_id: ObjectId,
    pub my_collection_id: ObjectId,
}

pub fn me_too_schema() -> Document {
    doc! {
        "$jsonSchema": doc! {
            "bsonType": "object",
            "title": "MeToo Object Validation",
            "required": ["too_id", "my_collection_id"],
            "properties": doc! {
                "too_id": {
                    "bsonType": "objectId",
                    "description": "must be a objectId and is required"
                },
                "my_collection_id": {
                    "bsonType": "objectId",
                    "description": "must be a objectId and is required"
                }
            }
        }
    }
}
