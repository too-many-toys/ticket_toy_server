use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub uid: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub age: Option<UserAge>,
    pub account_type: UserAccountType,
}

impl From<User> for Bson {
    fn from(user: User) -> Bson {
        mongodb::bson::to_bson(&user).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserAge {
    Age10,
    Age20,
    Age30,
    Age40,
    Age50,
    Age60,
    Age70,
    Age80,
    Age90,
    Age100,
}
impl From<UserAge> for Bson {
    fn from(a: UserAge) -> Bson {
        mongodb::bson::to_bson(&a).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserAccountType {
    Kakao,
    Naver,
    Google,
    Apple,
    Service,
}
impl From<UserAccountType> for Bson {
    fn from(t: UserAccountType) -> Bson {
        mongodb::bson::to_bson(&t).unwrap()
    }
}
