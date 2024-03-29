use crate::api::api_routes;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

pub fn app() -> Router {
    let middleware = ServiceBuilder::new().compression();

    Router::new()
        .nest("/api/v1", api_routes())
        .layer(middleware)
}
