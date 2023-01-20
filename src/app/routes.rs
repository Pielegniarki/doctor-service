use axum::response::IntoResponse;

pub mod authentication;
pub mod doctors;
pub mod healthcheck;
pub mod rating;
pub mod notification;
pub mod specialties;

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}