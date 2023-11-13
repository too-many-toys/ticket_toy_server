use axum::{
    routing::{get, post},
    Router,
};

use crate::config::AppState;

pub mod movie_api;
pub mod oauth;
pub mod user;

pub fn movie_routes() -> Router<AppState> {
    Router::new()
        .route("/list", get(movie_api::movie_list::get))
        .route("/posters", get(movie_api::posters::get))
        .route("/:movie_id", get(movie_api::movie::get))
        .route("/i", post(movie_api::movie::insert))
        .route("/credits/:movie_id", get(movie_api::credits::get))
}

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/signin", post(user::signin))
}

pub fn oauth_routers() -> Router<AppState> {
    Router::new()
        .route("/kakao/login", get(oauth::kakao_login))
        .route("/kakao/redirect", get(oauth::kakao_redirect))
}
