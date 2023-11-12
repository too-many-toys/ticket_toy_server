use axum::{extract::State, Json};
use mongodb::{bson::doc, options::UpdateOptions};
use serde::{Deserialize, Serialize};

use crate::{auth, config::UserState, errors::AppError, model::user::UserAccountType};

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub uid: String,
    pub account_type: UserAccountType,
}

#[derive(Serialize, Deserialize)]
pub struct SigninResponse {
    pub token: String,
}

pub async fn signin(
    State(user_state): State<UserState>,
    Json(payload): Json<UserId>,
) -> Result<Json<SigninResponse>, AppError> {
    let options = UpdateOptions::builder().upsert(Some(true)).build();
    let result = user_state
        .collection
        .update_one(
            doc! {"uid": payload.uid.clone()},
            doc! {"$set": {"uid": payload.uid.clone(), "account_type": payload.account_type.clone()}},
            options,
        )
        .await;
    if let Err(e) = result {
        return Err(AppError::Api(e.to_string()));
    }

    let jwt = auth::create(payload.uid, 2592000);

    Ok(Json(SigninResponse {
        token: jwt.unwrap(),
    }))
}

pub async fn set_user_info() {}

// pub async fn movies(headers: HeaderMap) {
//     let authorization = headers
//         .get("Authorization")
//         .ok_or(AppError::UserApi(UserApiError::JWT(
//             "not exist authorization".to_string(),
//         )))?
//         .to_str()
//         .unwrap()
//         .to_string();
// }
