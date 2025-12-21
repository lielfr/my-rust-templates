use axum::{http::StatusCode, response::IntoResponse, routing::get};

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

pub(crate) fn get_router() -> axum::Router {
    axum::Router::new().route("/healthz", get(health_check))
}
