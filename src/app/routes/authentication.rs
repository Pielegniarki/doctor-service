use std::sync::Arc;

use argon2::{password_hash::{SaltString, rand_core::OsRng}, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use axum::{extract::State, response::IntoResponse, Json};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::{app::AppState, db::schemas::{Doctor, Credential}};

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: Option<String>
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(params): Json<LoginParams>,
) -> impl IntoResponse {
    let credentials = state.db.collections().credential();

    let result = credentials
        .find_one(doc!{ "email": params.email }, None)
        .await
        .unwrap();

    Json(match result {
        Some(credential) => {
            let hash = PasswordHash::new(&credential.password).unwrap();

            let argon2 = Argon2::default();

            if argon2.verify_password(params.password.as_bytes(), &hash).is_ok() {
                LoginResponse { id: Some(credential.doctor_id.to_hex()) }
            } else {
                LoginResponse { id: None }
            }
        },
        None => LoginResponse { id: None }
    })
}

#[derive(Deserialize)]
pub struct RegisterParams {
    pub name: String,
    pub email: String,
    pub password: String,
    pub specialties: Vec<String>
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub ok: bool
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(params): Json<RegisterParams>,
) -> impl IntoResponse {
    let doctors = state.db.collections().doctor();
    let credentials = state.db.collections().credential();

    let specialties = params.specialties.into_iter().map(|id| ObjectId::parse_str(id).unwrap()).collect::<Vec<_>>();

    let inserted = doctors
        .insert_one(Doctor {
            _id: ObjectId::new(),
            name: params.name,
            specialties
        }, None)
        .await
        .unwrap();


    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(params.password.as_bytes(), &salt).unwrap().to_string();

    credentials
        .insert_one(Credential {
            doctor_id: inserted.inserted_id.as_object_id().unwrap(),
            email: params.email,
            password: hash
        }, None)
        .await
        .unwrap();

    Json(RegisterResponse { ok: true })
}
