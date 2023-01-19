use std::{sync::Arc, str::FromStr};

use axum::{extract::{State, Query}, response::IntoResponse, Json, http::HeaderMap};
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;

use crate::app::AppState;

pub async fn get_info(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let doctors = state.db.collections().doctor();

    let id = headers.get("X-PLG-ID").unwrap().to_str().unwrap();

    let Ok(id) = ObjectId::from_str(id) else {
        return Json(Result::Err("Cannot parse ObjectID"));
    };

    let result = doctors.find_one(doc! { "_id": id }, None).await.expect("Connection error");

    Json(match result {
        Some(doctor) => Result::Ok(doctor),
        None => Result::Err("No doctor in database with such ID")
    })
}
