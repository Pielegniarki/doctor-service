use app::App;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

mod app;
mod db;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let db = db::DB::new("mongodb://pielegniarki:pielegniarki@localhost:27017").await?;

    let client = reqwest::Client::new();

    // let doctor = db.collections().doctor();

    // doctor.insert_one(Doctor{name: s!("Hello, world!")}, None).await?;

    
    App::serve(db, client).await?;

    Ok(())
}