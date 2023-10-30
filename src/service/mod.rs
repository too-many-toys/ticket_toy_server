use axum::{routing::get, Router};

use crate::config::AppState;

pub mod movie;
pub mod user;

pub fn movie_routes() -> Router<AppState> {
    Router::new().route("/movies", get(movie::get_movies))
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/kakao/login", get(user::login))
        .route("/kakao/redirect", get(user::redirect))
}
