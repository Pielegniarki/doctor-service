use std::sync::Arc;

use axum::{Form, response::{IntoResponse, Html}, extract::State};

use crate::{db::schemas::Prescription, app::AppState};

pub async fn get() -> impl IntoResponse {
    Html(include_str!("../views/issuePrescription.html"))
}

pub async fn post(
    State(state): State<Arc<AppState>>,
    Form(body): Form<Prescription>,
) -> impl IntoResponse {
    // TODO: fetch patient ip
    state
        .http_client
        .post("/issuePrescription/check")
        .json::<Prescription>(&Prescription {
            medicine: body.medicine,
            description: body.description,
            doctor: body.doctor,
        })
        .send()
        .await
        .unwrap();

    "Ok"
}
