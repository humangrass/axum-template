use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub mod hello;

#[derive(OpenApi)]
#[openapi(
    paths(
        hello::world
    ),
    tags(
        (name = "hello", description = "Hello management"),
    )
)]
struct ApiDoc;

pub fn create_router() -> Router {
    let api_router = Router::new()
        .nest("/api/hello", hello::router());

    Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .merge(api_router)
}
