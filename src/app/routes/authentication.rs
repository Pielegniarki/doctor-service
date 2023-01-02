use axum::{
    response::{Html, IntoResponse},
};
pub mod login {
    use super::*;

    pub async fn get() -> impl IntoResponse {
        Html(include_str!("../views/login.html"))
    }
}
