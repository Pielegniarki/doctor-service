use axum::response::IntoResponse;

pub async fn http() -> impl IntoResponse {
    "Server OK"
}