use axum::{routing::get, Router};

use crate::config::AppState;

pub mod movie;

pub fn movie_routes() -> Router<AppState> {
    Router::new().route("/movies", get(movie::get_movies))
}
