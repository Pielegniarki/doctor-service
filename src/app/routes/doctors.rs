use std::{sync::Arc, str::FromStr};

use axum::{extract::{State, Query}, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{app::{AppState, types::JsResponse}, db::schemas::Doctor};

#[derive(Deserialize)]
pub struct GetInfoParams { 
    pub id: String
}

pub async fn get_info(
    Query(params): Query<GetInfoParams>,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let doctors = state.db.collections().doctor();

    let Ok(id) = mongodb::bson::oid::ObjectId::from_str(&params.id) else {
        return Json(JsResponse::Err("Cannot parse ObjectID"));
    };

    let result = doctors.find_one(doc! { "_id": id }, None).await.expect("Connection error");

    Json(match result {
        Some(doctor) => JsResponse::Ok(doctor),
        None => JsResponse::Err("No doctor in database with such ID")
    })
}
