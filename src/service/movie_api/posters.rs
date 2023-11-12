pub use axum::extract::{Json, Path, Query, State};
pub use reqwest;
pub use std::collections::HashMap;

use crate::{config::MovieState, errors::movie::MovieApiError, errors::AppError, request};

pub async fn get(
    Query(query): Query<HashMap<String, String>>,
    State(movie_state): State<MovieState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let movie_id = if let Some(id) = query.get("movie_id") {
        id
    } else {
        return Err(AppError::MovieApi(MovieApiError::Input(
            "movie_id".to_string(),
            None,
        )));
    };

    let posters = request::get::<serde_json::Value>(format!(
        "https://api.themoviedb.org/3/movie/{}/images?api_key={}",
        movie_id, movie_state.api_key
    ))
    .await?;

    Ok(Json(posters))
}
