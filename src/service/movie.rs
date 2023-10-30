use axum::extract::{Query, State};
use reqwest;
use std::collections::HashMap;

use crate::{config::MovieState, errors::movie::MovieAPIError, errors::AppError};

pub async fn get_movies<'a>(
    Query(query): Query<HashMap<String, String>>,
    State(movie_state): State<MovieState>,
) -> Result<String, AppError<'a>> {
    let search_keyword = if let Some(keyword) = query.get("search") {
        keyword
    } else {
        return Err(AppError::MovieAPI(MovieAPIError::Input("search", None)));
    };
    let page = if let Some(page) = query.get("page") {
        page
    } else {
        "1"
    };

    let movies = reqwest::get(format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}&include_adult=false&language=ko-kr&page={}",
        movie_state.api_key, search_keyword, page
    ))
    .await
    .unwrap()
    .text()
    .await;
    if let Err(e) = movies {
        return Err(AppError::MovieAPI(MovieAPIError::API(e.to_string())));
    }

    Ok(movies.unwrap())
}
