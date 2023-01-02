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
    // state
    //     .http_client
    //     .post("pacjent.pielegniarki.org/issuePrescription")
    //     .json::<Prescription>(&body)
    //     .send()
    //     .await
    //     .unwrap();

    "Ok"
}
