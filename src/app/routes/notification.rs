use std::sync::Arc;

use axum::{Json, response::IntoResponse, extract::State};

use crate::{db::schemas::Notification, app::AppState};

pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Notification>,
) -> impl IntoResponse {
    state.db.collections().notification().insert_one(body, None).await.unwrap();

    "Ok"
}
