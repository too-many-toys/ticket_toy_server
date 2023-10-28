use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub mod movie;

pub enum AppError<'a> {
    MovieAPI(movie::MovieAPIError<'a>),
}

impl<'a> From<movie::MovieAPIError<'a>> for AppError<'a> {
    fn from(inner: movie::MovieAPIError<'a>) -> Self {
        AppError::MovieAPI(inner)
    }
}

impl<'a> IntoResponse for AppError<'a> {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MovieAPI(movie::MovieAPIError::API(e)) => {
                tracing::error!("Call API failed: {}", e);
                (StatusCode::EXPECTATION_FAILED, json!({"msg": "api error"}))
            }
            AppError::MovieAPI(movie::MovieAPIError::Input(input, value)) => (
                StatusCode::BAD_REQUEST,
                json!({"msg": "Invalid input", "input": input, "value": value}),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
