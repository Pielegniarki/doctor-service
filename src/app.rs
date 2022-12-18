use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};

use crate::db::DB;

mod routes;

pub struct AppState {
    db: DB,
    http_client: reqwest::Client
}

pub struct App;

impl App {
    fn healthcheck_api() -> Router<Arc<AppState>> {
        Router::new()
            .route("/http", get(routes::healthcheck::http))
            .route("/db", get(routes::healthcheck::db))
    }

    fn prescription_api() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", 
                get(routes::issue_prescription::get)
                .post(routes::issue_prescription::post)
            )
    }

    fn rating_api() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::rating::post))
    }

    fn notification_api() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::notification::post))
    }

    fn authentication_api() -> Router<Arc<AppState>> {
        Router::new()
            .route("/login", get(routes::authentication::login::get))
    }

    pub async fn serve(db: DB, http_client: reqwest::Client) -> anyhow::Result<()> {
        let state = Arc::new(AppState { db, http_client });

        let app = Router::new()
            .route("/", get(routes::index))
            .nest("/healthcheck", App::healthcheck_api())
            .nest("/issuePrescription", App::prescription_api())
            .nest("/rating", App::rating_api())
            .nest("/notify", App::notification_api())
            .nest("/auth", App::authentication_api())
            .with_state(state);

        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
