use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};

use crate::db::DB;
use crate::http_client::HttpClient;

mod routes;
mod types;

pub struct AppState {
    db: DB,
    http_client: HttpClient
}

pub struct App;

impl App {
    pub async fn serve(db: DB, http_client: HttpClient) -> anyhow::Result<()> {
        let state = Arc::new(AppState { db, http_client });

        let app = Router::new()
            .route("/", get(routes::index))
            .nest("/healthcheck", api::healthcheck())
            .nest("/doctors", api::doctors())
            .nest("/issuePrescription", api::prescription())
            .nest("/rating", api::rating())
            .nest("/notify", api::notification())
            .nest("/auth", api::authentication())
            .with_state(state);

        #[cfg(debug_assertions)]
        let app = app.layer(tower_http::cors::CorsLayer::permissive());

        let addr = SocketAddr::from(([0, 0, 0, 0], 4000));

        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}


mod api {
    use super::*;

    pub fn healthcheck() -> Router<Arc<AppState>> {
        Router::new()
            .route("/http", get(routes::healthcheck::http))
            .route("/db", get(routes::healthcheck::db))
    }

    pub fn doctors() -> Router<Arc<AppState>> {
        Router::new()
            .route("/getInfo", get(routes::doctors::get_info))
    }

    pub fn prescription() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", 
                get(routes::issue_prescription::get)
                .post(routes::issue_prescription::post)
            )
    }

    pub fn rating() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::rating::post))
    }

    pub fn notification() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::notification::post))
    }

    pub fn authentication() -> Router<Arc<AppState>> {
        Router::new()
            .route("/login", get(routes::authentication::login::get))
    }
}