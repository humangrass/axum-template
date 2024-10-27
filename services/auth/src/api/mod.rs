use std::sync::Arc;
use axum::{Extension, Router};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use crate::app::AppState;

pub mod users;

#[derive(OpenApi)]
#[openapi(
    paths(
        users::create_user
    ),
    tags(
        (name = "users", description = "User management")
    )
)]
struct ApiDoc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new()
        .nest("/api/users", users::router());

    Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .merge(api_router)
        .layer(Extension(app_state))
}
