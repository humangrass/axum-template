use axum::{Router, routing::post, extract::{Extension, Json}, http::StatusCode};
use std::sync::Arc;
use crate::app::AppState;
use crate::structs::error::ErrorResponse;
use crate::structs::users::CreateUserRequest;

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
    (status = 201, description = "User created successfully"),
    (status = 400, description = "Invalid data or user already exists", body = ErrorResponse),
    (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_error) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            message: validation_error,
        })));
    }

    let user = payload.model();

    match state.create_user(user).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(err) if err.to_string().contains("unique constraint") => {
            Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
                message: "Username or email already exists.".to_string(),
            })))
        }
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            message: "Internal server error.".to_string(),
        }))),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_user))
}
