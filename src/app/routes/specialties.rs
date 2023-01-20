use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use futures::TryStreamExt;

use crate::app::AppState;

pub async fn list_all(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let specialties = state.db.collections().specialty();


    let result = match specialties.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Json(vec![])
    };

    let Ok(specialties) = result.try_collect().await else {
        return Json(vec![])
    };


    Json(specialties)
}
