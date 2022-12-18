use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    Form,
};

use crate::{app::AppState, db::schemas::Prescription};

pub mod login {
    use super::*;

    pub async fn get() -> impl IntoResponse {
        Html(include_str!("../views/login.html"))
    }

    // pub async fn post(
    //     State(state): State<Arc<AppState>>,
    //     Form(body): Form<Prescription>,
    // ) -> impl IntoResponse {
    //     // TODO: fetch patient ip
    //     state
    //         .http_client
    //         .post("pacjent.pielegniarki.org/issuePrescription")
    //         .json::<Prescription>(&body)
    //         .send()
    //         .await
    //         .unwrap();

    //     "Ok"
    // }
}
