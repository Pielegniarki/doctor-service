use axum::{
    response::IntoResponse,
};


pub mod login {
    use std::sync::Arc;

    use axum::{extract::{Query, State}, Json};
    use mongodb::bson::doc;
    use serde::{Serialize, Deserialize};

    use crate::app::AppState;

    use super::*;

    #[derive(Deserialize)]
    pub struct LoginParams {
        pub email: String,
        pub password: String
    }

    #[derive(Serialize)]
    pub struct LoginResult<'a> {
        pub id: &'a str
    }

    pub async fn post(
        Query(params): Query<LoginParams>,
        State(state): State<Arc<AppState>>
    ) -> impl IntoResponse{
        let doctors = state.db.collections().doctor();

        let Ok(doctor) = doctors.find_one(doc! {"email": params.email, "password": params.password}, None).await else {
           return Json(Err("Connection error"));
        };

        Json(doctor.ok_or("Didn't find doctor with provided email and password"))
    }
}
