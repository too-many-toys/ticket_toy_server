use axum::{
    extract::{Query, State},
    routing::{get, Router},
};
use reqwest;
use std::{collections::HashMap, sync::Arc};

use crate::{errors::movie::MovieAPIError, errors::AppError, state::MovieState};

pub async fn get_movies<'a>(
    Query(query): Query<HashMap<String, String>>,
    State(state): State<Arc<MovieState>>,
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
        state.movie_api_key, search_keyword, page
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

pub fn movie_routes(state: Arc<MovieState>) -> Router {
    Router::new()
        .route("/movies", get(get_movies))
        .with_state(state)
}
