use std::{sync::Arc, str::FromStr};

use axum::{extract::{State, Query}, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde::Deserialize;

use futures::{TryStreamExt};

use crate::{app::AppState};

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
        return Json(Err("Cannot parse ObjectID"));
    };

    let result = doctors.find_one(doc! { "_id": id }, None).await.expect("Connection error");

    Json(match result {
        Some(doctor) => Ok(doctor),
        None => Err("No doctor in database with such ID")
    })
}


pub async fn list_all(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let doctors = state.db.collections().doctor();


    let result = match doctors.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Json(vec![])
    };

    let v: Vec<_> = result.try_collect().await.expect("XD");


    Json(v)
}
