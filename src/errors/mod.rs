use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub mod movie;
pub mod user;

pub enum AppError {
    MovieApi(movie::MovieApiError),
    UserApi(user::UserApiError),
}

impl<'a> From<movie::MovieApiError> for AppError {
    fn from(inner: movie::MovieApiError) -> Self {
        AppError::MovieApi(inner)
    }
}

impl<'a> IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MovieApi(movie::MovieApiError::API(e)) => {
                tracing::error!("Call movie API failed: {}", e);
                (
                    StatusCode::EXPECTATION_FAILED,
                    json!({"msg": "movie api error"}),
                )
            }
            AppError::MovieApi(movie::MovieApiError::Input(input, value)) => (
                StatusCode::BAD_REQUEST,
                json!({"msg": "Invalid input", "input": input, "value": value}),
            ),
            AppError::UserApi(user::UserApiError::JWT(e)) => {
                tracing::error!("Call user API failed: {}", e);
                (StatusCode::EXPECTATION_FAILED, json!({"msg": "jwt error"}))
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
