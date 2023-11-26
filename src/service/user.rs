use std::str::FromStr;

use axum::{
    extract::{Multipart, Path, Query, State},
    Extension, Json,
};
use chrono::Utc;
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

#[derive(Deserialize, Debug)]
pub struct ReqModifyMyCollections {
    pub rating: f64,
    pub content: String,
    pub is_post: bool,
}

#[derive(Serialize, Default, Debug)]
pub struct ResGetMyCollections {
    pub collections: Vec<RMyCollection>,
}

#[derive(Serialize, Default, Debug)]
pub struct RMyCollection {
    pub id: String,
    pub image_url: String,
}
impl From<MyCollection> for RMyCollection {
    fn from(mc: MyCollection) -> Self {
        Self {
            id: mc.id.unwrap().to_string(),
            image_url: mc.image_url,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RCollectionDetail {
    pub id: String,
    pub author_id: String,
    pub movie_id: i64,
    pub movie_title: String,
    pub rating: f64,
    pub content: String,
    pub is_post: bool,
    pub me_too: u64,
    pub watched_at: Option<bson::DateTime>,
}
impl From<MyCollection> for RCollectionDetail {
    fn from(mc: MyCollection) -> Self {
        Self {
            id: mc.id.unwrap().to_string(),
            author_id: mc.author_id.unwrap().to_string(),
            movie_id: mc.movie_id.unwrap(),
            movie_title: mc.movie_title.unwrap(),
            rating: mc.rating.unwrap(),
            content: mc.content.unwrap(),
            is_post: mc.is_post.unwrap(),
            me_too: mc.me_too.unwrap(),
            watched_at: mc.watched_at,
        }
    }
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
    my_collection.author_id = Some(ObjectId::from_str(token.sub.clone().as_str()).unwrap());
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "movie_id" => {
                my_collection.movie_id = Some(field.text().await.unwrap().parse().unwrap());
            }
            "movie_title" => {
                my_collection.movie_title = Some(field.text().await.unwrap());
            }
            "is_post" => {
                my_collection.is_post = Some(field.text().await.unwrap().parse().unwrap());
            }
            "rating" => my_collection.rating = Some(field.text().await.unwrap().parse().unwrap()),
            "content" => my_collection.content = Some(field.text().await.unwrap()),
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
            "watch_at" => {
                let watch_at: chrono::DateTime<Utc> = field.text().await.unwrap().parse().unwrap();
                my_collection.watched_at = Some(bson::DateTime::from_chrono(watch_at));
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
    Path(id): Path<String>,
    State(mc_state): State<MyCollectionState>,
) -> Result<Json<RCollectionDetail>, AppError> {
    let result = if let Ok(r) = mc_state
        .collection
        .find_one(
            doc! {"_id": ObjectId::from_str(id.as_str()).unwrap(), "author_id": ObjectId::from_str(token.sub.as_str()).unwrap()},
            None,
        )
        .await
    {
        tracing::info!("get_my_collection: {:?}", r);
        r.unwrap()
    } else {
        return Err(AppError::Api("".to_string()));
    };

    Ok(Json(RCollectionDetail::from(result)))
}

pub async fn get_my_collections(
    Extension(token): Extension<Claims>,
    State(mc_state): State<MyCollectionState>,
    Query(payload): Query<ReqGetMyCollections>,
) -> Result<Json<ResGetMyCollections>, AppError> {
    let skip = payload.skip;
    let opt = FindOptions::builder()
        .projection(doc! {
            "_id": 1,
            "image_url": 1,
        })
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
        collections: Vec::<RMyCollection>::new(),
    };

    while let Some(r) = cursor.try_next().await.unwrap() {
        result.collections.push(RMyCollection::from(r));
    }

    Ok(Json(result))
}

// pub async fn modify_my_collection(
//     Extension(token): Extension<Claims>,
//     State(mc_state): State<MyCollectionState>,
//     Json(payload): Json<ReqGetMyCollections>,
// ) {
//     let result = mc_state
//         .collection
//         .update_one(
//             doc! {"id": id, "author_id": ObjectId::from_str(token.sub.as_str()).unwrap()},
//             doc! {"$set": {"uid": payload.uid.clone(), "account_type": payload.account_type.clone()}},
//             None,
//         )
//         .await;
//     if let Err(e) = result {
//         return Err(AppError::Api(e.to_string()));
//     }
// }
