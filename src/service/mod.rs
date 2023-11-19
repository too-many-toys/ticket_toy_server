use axum::{
    http::{HeaderMap, Request},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};

use crate::{auth::verify, config::AppState, errors::AppError};

pub mod movie_api;
pub mod user;

pub fn movie_routes() -> Router<AppState> {
    Router::new()
        .route("/list", get(movie_api::movie_list::get))
        .route("/posters", get(movie_api::posters::get))
        .route("/:movie_id", get(movie_api::movie::get))
        // .route("/i", post(movie_api::movie::insert))
        .route("/credits/:movie_id", get(movie_api::credits::get))
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        // Multipart
        .route("/collection", post(user::put_my_collection))
        .route("/collection", get(user::get_my_collection))
        .route("/collections", get(user::get_my_collections))
        .route_layer(middleware::from_fn(auth_middleware))
        .route("/signin", post(user::signin))
}

async fn auth_middleware<B>(
    headers: HeaderMap,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let bearer = if let Some(h) = headers.get("Authorization") {
        h.to_str().unwrap()
    } else {
        return Err(AppError::Auth());
    };
    let bearer = bearer.replace("Bearer ", "");

    let token = match verify(bearer) {
        Ok(t) => t,
        Err(_) => return Err(AppError::Auth()),
    };

    request.extensions_mut().insert(token);

    let response = next.run(request).await;

    Ok(response)
}
