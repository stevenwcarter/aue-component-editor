use axum::response::IntoResponse;
use axum::Json;
use axum::{http::StatusCode, routing::get, Router};
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

pub fn middleware() -> tower::ServiceBuilder<
    tower::layer::util::Stack<
        tower_http::compression::CompressionLayer,
        tower::layer::util::Identity,
    >,
> {
    ServiceBuilder::new().compression()
}

pub fn api_routes() -> Router {
    Router::new().route("/test", get(get_test))
}

pub async fn get_test() -> &'static str {
    " hello world"
}

pub fn err_wrapper<T: Serialize>(result: anyhow::Result<T>) -> impl IntoResponse {
    Json(
        result
            .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))
            .unwrap(),
    )
}
