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
            .route("/http", get(routes::http_healthcheck))
            .route("/db", get(routes::db_healthcheck))
    }

    fn app_api() -> Router<Arc<AppState>> {
        Router::new()
            .route("/issuePrescription", 
                get(routes::get_issue_prescription)
                .post(routes::post_issue_prescription)
            )
    }

    pub async fn serve(db: DB, http_client: reqwest::Client) -> anyhow::Result<()> {


        let state = Arc::new(AppState { db, http_client });

        let app = Router::new()
            .route("/", get(routes::index))
            .nest("/healthcheck", App::healthcheck_api())
            .with_state(state);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        tracing::debug!("listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
