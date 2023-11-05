use axum::{extract::State, http::HeaderMap, Json};
use mongodb::{bson::doc, options::FindOneAndUpdateOptions};
use serde::{Deserialize, Serialize};

use crate::{auth, config::UserState, errors::AppError};

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub user_id: String,
}

pub async fn signin(
    headers: HeaderMap,
    State(user_state): State<UserState>,
    Json(payload): Json<UserId>,
) -> Result<String, AppError> {
    // let authorization = headers
    //     .get("Authorization")
    //     .ok_or(AppError::UserApi(UserApiError::JWT(
    //         "not exist authorization".to_string(),
    //     )))?
    //     .to_str()
    //     .unwrap()
    //     .to_string();

    let options = FindOneAndUpdateOptions::builder()
        .upsert(Some(true))
        .build();
    user_state
        .collection
        .find_one_and_update(
            doc! {"uid": payload.user_id.clone()},
            doc! {"$set": {"uid": payload.user_id.clone()}},
            options,
        )
        .await
        .unwrap();

    let jwt = auth::create(payload.user_id, 2592000);

    Ok(jwt.unwrap())
}
