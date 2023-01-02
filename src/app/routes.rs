use axum::response::IntoResponse;

pub mod authentication;
pub mod doctors;
pub mod healthcheck;
pub mod issue_prescription;
pub mod rating;
pub mod notification;

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}