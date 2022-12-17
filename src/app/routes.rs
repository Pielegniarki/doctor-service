use std::sync::Arc;

use axum::{response::IntoResponse, extract::State, Json};
use serde::Deserialize;

use super::AppState;

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn http_healthcheck() -> impl IntoResponse {
    "Server OK"
}

#[axum_macros::debug_handler]
pub async fn db_healthcheck(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let doctors = state.db.collections().doctor();

    doctors.name().to_owned()
}

pub async fn get_issue_prescription() -> impl IntoResponse {
    include_str!("./views/issuePrescription.html")
}

pub async fn get_issue_prescription_interceptor() -> impl IntoResponse {
    include_str!("./views/issuePrescription.html")
}

#[derive(Deserialize)]
pub struct IssuePrescriptionIn {
    name: String,
    description: String
}

pub async fn post_issue_prescription(
    State(state): State<Arc<AppState>>,
    Json(body): Json<IssuePrescriptionIn>
) -> impl IntoResponse {
    // TODO: fetch patient ip
    state.http_client.post("patient.ip/issuePrescription").send().await.unwrap();


    "Ok"
}