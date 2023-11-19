use std::str::FromStr;

use std::path::Path;

use axum::{
    body::Bytes,
    extract::{Multipart, State},
    Extension, Json,
};
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use tokio::fs::write;

use crate::{
    auth,
    config::UserState,
    errors::AppError,
    model::{
        my_collection::MyCollection,
        user::{Claims, User, UserAccountType},
    },
};

pub struct ReqMyCollection {
    movie_id: i64,
    movie_title: String,
    is_post: Option<bool>,
    watched_at: Option<String>,
    rating: Option<f64>,
    content: Option<String>,
    image: Bytes,
}

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
    let mut my_collection = MyCollection::default();
    my_collection.author_id = ObjectId::from_str(token.sub.clone().as_str()).unwrap();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "movie_id" => {
                my_collection.movie_id = field.text().await.unwrap().parse().unwrap();
            }
            "movie_title" => {
                my_collection.movie_title = field.text().await.unwrap();
            }
            // TODO: 컬렉션에서 바로 포스트로 올리는 기능 출시하면 추가
            // "is_post" => {
            //     let is_post = field.text().await.unwrap();
            // }
            "rating" => {
                if let Ok(r) = field.text().await {
                    if let Ok(s) = r.parse() {
                        my_collection.rating = Some(s);
                    }
                }
            }
            "content" => {
                if let Ok(r) = field.text().await {
                    my_collection.content = Some(r);
                }
            }
            "image" => {
                let folder_name = sha256::digest(token.sub.clone());
                let path = format!("/usr/local/var/images/{}", folder_name);
                let filename = uuid::Uuid::new_v4();
                let url = format!("{}.jpg", filename);

                if !Path::new(&path).exists() {
                    tokio::fs::create_dir(&path).await.unwrap();
                }
                let result = write(
                    format!("/usr/local/var/images/{}/{}", folder_name, url.clone()),
                    field.bytes().await.unwrap(),
                )
                .await;
                if let Err(e) = result {
                    return Err(AppError::Api(e.to_string()));
                }

                my_collection.image_url = url;
            }
            _ => {}
        }
    }

    Ok("asdf".to_string())
}
