use axum::{routing::get, Router};

use crate::config::AppState;

pub mod movie;
pub mod oauth;
pub mod user;

pub fn movie_routes() -> Router<AppState> {
    Router::new().route("/movies", get(movie::get_movies))
}

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/signin", get(user::signin))
}

pub fn oauth_routers() -> Router<AppState> {
    Router::new()
        .route("/kakao/login", get(oauth::kakao_login))
        .route("/kakao/redirect", get(oauth::kakao_redirect))
}
