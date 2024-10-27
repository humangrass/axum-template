use axum::{Router, routing::post, extract::{Extension, Json}, http::StatusCode};
use std::sync::Arc;
use models::user::User;
use service::auth::AuthState;
use crate::structs::users::CreateUserRequest;

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
    (status = 201, description = "User created successfully", body = CreateUserRequest),
    (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user(
    Extension(state): Extension<Arc<AuthState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let user = payload.model();

    let created_user = state.create_user(user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(created_user))
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_user))
}
