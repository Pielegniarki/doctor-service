use axum::response::IntoResponse;

pub mod authentication;
pub mod doctors;
pub mod healthcheck;
pub mod notification;
pub mod rating;
pub mod specialties;

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}