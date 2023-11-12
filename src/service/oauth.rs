use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use mongodb::{bson::doc, options::UpdateOptions};
use serde::{Deserialize, Serialize};

use crate::{auth, config::UserState, errors::AppError};

#[derive(Deserialize, Serialize)]
pub struct KakaoOauth {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub refresh_token_expires_in: i32,
}

#[derive(Deserialize)]
pub struct Querys {
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct KakaoBody {
    pub grant_type: String,
    pub client_id: String,
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserBody {
    pub target_id_type: String,
    pub target_id: String,
}
// {
//     "access_token": "u8kEAwr09sHWsz7KO2Ury7grmeAdUfVDsREKKiVQAAABi8NYvT-xu3fh8M0xkQ",
//     "token_type": "bearer",
//     "refresh_token": "toc-IDHEeJ7QEuBJWqnNUtiWxh52wC5Aph0KKiVQAAABi8NYvTyxu3fh8M0xkQ",
//     "expires_in": 21599,
//     "refresh_token_expires_in": 5183999
//     }

#[derive(Serialize, Deserialize, Debug)]
pub struct Access {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SigninResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfoRes {
    pub id: i64,
}

pub async fn kakao_login(State(user_state): State<UserState>) -> impl IntoResponse {
    Redirect::permanent(&format!(
        "https://kauth.kakao.com/oauth/authorize?client_id={}&redirect_uri={}&response_type=code",
        user_state.kakao_api_key, user_state.kakao_redirect_url
    ))
}

pub async fn kakao_redirect(
    query: Option<Query<Querys>>,
    State(user_state): State<UserState>,
) -> Result<Json<SigninResponse>, AppError> {
    let Query(query) = query.unwrap();
    let client_id = std::env::var("KAKAO_API_KEY").expect("no kakao api key");

    let body = serde_urlencoded::to_string(&KakaoBody {
        grant_type: String::from("authorization_code"),
        client_id,
        code: query.code,
        redirect_uri: user_state.kakao_redirect_url,
    })
    .expect("serialize fail");

    let client = reqwest::Client::new();
    let res = client
        .post("https://kauth.kakao.com/oauth/token")
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap()
        .json::<Access>()
        .await
        .unwrap();

    let access_token = res.access_token;

    let client = reqwest::Client::new();
    let res = client
        .get("https://kapi.kakao.com/v2/user/me")
        .header("Authorization", format!("Bearer {}", access_token))
        .header(
            "Content-type",
            "application/x-www-form-urlencoded;charset=utf-8",
        )
        .send()
        .await
        .unwrap()
        .json::<UserInfoRes>()
        .await
        .unwrap();

    let options = UpdateOptions::builder().upsert(Some(true)).build();
    let result = user_state
        .collection
        .update_one(
            doc! {"uid": res.id.to_string()},
            doc! {"$set": {"uid": res.id.to_string(), "account_type": "Kakao"}},
            options,
        )
        .await;
    if let Err(e) = result {
        return Err(AppError::Api(e.to_string()));
    }

    let jwt = auth::create(res.id.to_string(), 2592000);

    Ok(Json(SigninResponse {
        token: jwt.unwrap(),
    }))
}

/*
    이하 서버에서 강제로 실행시키기 위한 API
    평소엔 클라이언트에서 직접 KAKAO API를 사용하지만
    admin api를 사용할 일이 있을 경우 사용
*/

pub async fn kakao_logout(Json(payload): Json<UserId>) -> impl IntoResponse {
    let body = serde_urlencoded::to_string(&UserBody {
        target_id_type: String::from("user_id"),
        target_id: payload.user_id,
    })
    .expect("serialize fail");

    let client = reqwest::Client::new();
    let res = client
        .post("https://kapi.kakao.com/v1/user/logout")
        .header("Content-type", "application/x-www-form-urlencoded")
        .header(
            "Authorization",
            format!(
                "KakaoAK {}",
                std::env::var("KAKAO_ADMIN_KEY").expect("not exist admin key")
            ),
        )
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    (StatusCode::OK, Json(res))
}

pub async fn kakao_unlink_app(Json(payload): Json<UserId>) -> impl IntoResponse {
    let body = serde_urlencoded::to_string(&UserBody {
        target_id_type: String::from("user_id"),
        target_id: payload.user_id,
    })
    .expect("serialize fail");

    let client = reqwest::Client::new();
    let res = client
        .post("https://kapi.kakao.com/v1/user/unlink")
        .header("Content-type", "application/x-www-form-urlencoded")
        .header(
            "Authorization",
            format!(
                "KakaoAK {}",
                std::env::var("KAKAO_ADMIN_KEY").expect("not exist admin key")
            ),
        )
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    (StatusCode::OK, Json(res))
}
