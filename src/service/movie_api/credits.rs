pub use axum::extract::{Json, Path, Query, State};
pub use mongodb::bson::doc;
pub use reqwest;
pub use std::collections::HashMap;

use crate::{config::MovieState, errors::AppError, request};

pub async fn get(
    Path(movie_id): Path<String>,
    State(movie_state): State<MovieState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let credits = request::get::<serde_json::Value>(format!(
        "https://api.themoviedb.org/3/movie/{}/credits?api_key={}&language=ko-kr",
        movie_id, movie_state.api_key
    ))
    .await?;

    Ok(Json(credits))
}
