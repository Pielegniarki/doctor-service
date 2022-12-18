use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::app::AppState;

pub async fn http() -> impl IntoResponse {
    "Server OK"
}

#[axum_macros::debug_handler]
pub async fn db(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let doctors = state.db.collections().doctor();

    doctors.name().to_owned()
}
