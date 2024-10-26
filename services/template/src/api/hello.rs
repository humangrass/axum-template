use axum::Router;
use axum::routing::get;
use log::info;

#[utoipa::path(
    get,
    path = "/api/hello/world",
    responses(
        (status = 200, description = "Hello, world. Again!")
    )
)]
async fn world() -> &'static str {
    info!("Hello, world!");
    "Hello, world!"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(world))
}
