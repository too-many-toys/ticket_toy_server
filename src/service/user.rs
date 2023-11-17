use axum::{
    extract::{Multipart, State},
    Extension, Json,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{
    auth,
    config::UserState,
    errors::AppError,
    model::user::{Claims, User, UserAccountType},
};

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
    let user = user_state
        .collection
        .find_one(doc! {"uid": payload.uid.clone()}, None)
        .await;
    let user = if let Err(e) = user {
        return Err(AppError::Api(e.to_string()));
    } else {
        user.unwrap()
    };

    let user_id = if let None = user {
        let user = user_state
            .collection
            .insert_one(
                User {
                    id: None,
                    uid: payload.uid.clone(),
                    account_type: payload.account_type.clone(),
                    email: None,
                    nickname: None,
                    age: None,
                },
                None,
            )
            .await;
        let user = if let Err(e) = user {
            return Err(AppError::Api(e.to_string()));
        } else {
            user.unwrap()
        };

        user.inserted_id.to_string()
    } else {
        user.unwrap().id.unwrap().to_string()
    };

    let jwt = auth::create(user_id);

    Ok(Json(SigninResponse {
        token: jwt.unwrap(),
    }))
}

pub async fn set_user_info(State(user_state): State<UserState>, Json(payload): Json<UserId>) {
    // let result = user_state
    //     .collection
    //     .update_one(
    //         doc! {"uid": payload.uid.clone()},
    //         doc! {"$set": {"uid": payload.uid.clone(), "account_type": payload.account_type.clone()}},
    //         None,
    //     )
    //     .await;
    // if let Err(e) = result {
    //     return Err(AppError::Api(e.to_string()));
    // }
}

pub async fn put_my_collection(
    Extension(token): Extension<Claims>,
    State(user_state): State<UserState>,
    mut multipart: Multipart,
) -> Result<String, AppError> {
    tracing::info!("token: {:?}", token);
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }

    Ok("asdf".to_string())
}
