use axum::extract::{Query, State};
use reqwest;
use std::collections::HashMap;

use crate::{config::MovieState, errors::movie::MovieApiError, errors::AppError};

pub async fn movies<'a>(
    Query(query): Query<HashMap<String, String>>,
    State(movie_state): State<MovieState>,
) -> Result<String, AppError> {
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

    let movies = reqwest::get(format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}&include_adult=false&language=ko-kr&page={}",
        movie_state.api_key, search_keyword, page
    ))
    .await
    .unwrap()
    .text()
    .await;
    if let Err(e) = movies {
        return Err(AppError::MovieApi(MovieApiError::API(e.to_string())));
    }

    Ok(movies.unwrap())
}

pub async fn posters(
    Query(query): Query<HashMap<String, String>>,
    State(movie_state): State<MovieState>,
) -> Result<String, AppError> {
    let movie_id = if let Some(id) = query.get("movie_id") {
        id
    } else {
        return Err(AppError::MovieApi(MovieApiError::Input(
            "movie_id".to_string(),
            None,
        )));
    };

    let client = reqwest::Client::new();
    let posters = client
        .get(format!(
            "https://api.themoviedb.org/3/movie/{}/images?api_key={}",
            movie_id, movie_state.api_key
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await;
    if let Err(e) = posters {
        return Err(AppError::MovieApi(MovieApiError::API(e.to_string())));
    }
    Ok(posters.unwrap())
}
