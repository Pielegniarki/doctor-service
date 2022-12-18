use axum::response::IntoResponse;

pub mod authentication;
pub mod healthcheck;
pub mod issue_prescription;
pub mod rating;

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}