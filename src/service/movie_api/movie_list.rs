pub use axum::extract::{Json, Path, Query, State};
pub use reqwest;
pub use std::collections::HashMap;

use crate::{config::MovieState, errors::movie::MovieApiError, errors::AppError, request};

pub async fn get<'a>(
    Query(query): Query<HashMap<String, String>>,
    State(movie_state): State<MovieState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let search_keyword = if let Some(keyword) = query.get("search") {
        keyword
    } else {
        return Err(AppError::MovieApi(MovieApiError::Input(
            "search".to_string(),
            None,
        )));
    };
    let page = if let Some(page) = query.get("page") {
        page
    } else {
        "1"
    };

    let movies = request::get::<serde_json::Value>(format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}&include_adult=false&language=ko-kr&page={}",
        movie_state.api_key, search_keyword, page
    )).await?;

    Ok(Json(movies))
}
