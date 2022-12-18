use std::sync::Arc;

use axum::{Json, response::IntoResponse, extract::State};

use crate::{db::schemas::Rating, app::AppState};

pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Rating>,
) -> impl IntoResponse {
    // TODO: fetch patient ip
    state.db.collections().rating().insert_one(body, None).await.unwrap();

    "Ok"
}
