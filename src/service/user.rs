use std::str::FromStr;

use axum::{
    extract::{Multipart, State},
    Extension, Json,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};
use tokio::fs::write;

use crate::{
    auth,
    config::{MyCollectionState, UserState},
    errors::AppError,
    model::{
        my_collection::MyCollection,
        user::{Claims, User, UserAccountType},
    },
};

#[derive(Deserialize, Debug)]
pub struct ReqGetMyCollections {
    #[serde(default)]
    pub skip: u64,
}
impl Default for ReqGetMyCollections {
    fn default() -> Self {
        ReqGetMyCollections { skip: 0 }
    }
}

#[derive(Serialize, Default)]
pub struct ResGetMyCollections {
    pub collections: Vec<MyCollection>,
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
    State(mc_state): State<MyCollectionState>,
    mut multipart: Multipart,
) -> Result<Json<String>, AppError> {
    let mut my_collection = MyCollection::default();
    my_collection.author_id = ObjectId::from_str(token.sub.clone().as_str()).unwrap();
    while let Some(field) = multipart.next_field().await.unwrap() {
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
                let path = "/usr/local/var/images/";
                let filename = uuid::Uuid::new_v4();
                let url = format!("{}.jpg", filename);

                let result = write(
                    format!("{}/{}", path, url.clone()),
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

    if let Err(s) = mc_state.collection.insert_one(my_collection, None).await {
        return Err(AppError::Api(s.to_string()));
    }

    Ok(Json("OK".to_string()))
}

pub async fn get_my_collection(
    Extension(token): Extension<Claims>,
    State(mc_state): State<MyCollectionState>,
) -> Result<Json<MyCollection>, AppError> {
    let result = if let Ok(r) = mc_state
        .collection
        .find_one(
            doc! {"author_id": ObjectId::from_str(token.sub.as_str()).unwrap()},
            None,
        )
        .await
    {
        r.unwrap()
    } else {
        return Err(AppError::Api("".to_string()));
    };

    Ok(Json(result))
}
pub async fn get_my_collections(
    Extension(token): Extension<Claims>,
    State(mc_state): State<MyCollectionState>,
    Json(payload): Json<ReqGetMyCollections>,
) -> Result<Json<ResGetMyCollections>, AppError> {
    let skip = payload.skip;
    let opt = FindOptions::builder()
        .sort(doc! {"created_at": -1})
        .skip(skip)
        .limit(50)
        .build();
    let mut cursor = if let Ok(r) = mc_state
        .collection
        .find(
            doc! {"author_id": ObjectId::from_str(token.sub.as_str()).unwrap()},
            opt,
        )
        .await
    {
        r
    } else {
        return Err(AppError::Api("".to_string()));
    };

    let mut result = ResGetMyCollections {
        collections: Vec::<MyCollection>::new(),
    };

    while let Some(r) = cursor.try_next().await.unwrap() {
        result.collections.push(r);
    }

    Ok(Json(result))
}
