pub use axum::extract::{Json, Path, Query, State};
pub use mongodb::bson::doc;
pub use reqwest;
pub use std::collections::HashMap;

use crate::{
    config::MovieState,
    errors::{movie::MovieApiError, AppError},
    model::movie::Movie,
    request,
};

pub async fn get(
    Path(movie_id): Path<String>,
    State(movie_state): State<MovieState>,
) -> Result<Json<Movie>, AppError> {
    let movie = request::get::<Movie>(format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}&language=ko-kr",
        movie_id, movie_state.api_key
    ))
    .await?;

    Ok(Json(movie))
}

pub async fn insert(
    State(movie_state): State<MovieState>,
    Json(payload): Json<InsertRequest>,
) -> Result<Json<InsertResponse>, AppError> {
    let movie = request::get::<Movie>(format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}&language=ko-kr",
        payload.movie_id, movie_state.api_key
    ))
    .await?;

    let option = mongodb::options::UpdateOptions::builder()
        .upsert(Some(true))
        .build();
    let result = movie_state
        .collection
        .update_one(doc! {"id": movie.id}, doc! {"$set":movie}, option)
        .await;
    if let Err(e) = result {
        return Err(AppError::MovieApi(MovieApiError::API(e.to_string())));
    }

    Ok(Json(InsertResponse {
        msg: "success".to_string(),
    }))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct InsertRequest {
    pub movie_id: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct InsertResponse {
    pub msg: String,
}
