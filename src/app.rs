use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router, middleware::map_request_with_state,
};
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use token_extractor::token_map;

use crate::db::DB;
use crate::http_client::HttpClient;

mod routes;
mod token_extractor;

pub struct AppState {
    db: DB,
    http_client: HttpClient
}

pub struct App;

impl App {
    pub async fn serve(db: DB, http_client: HttpClient) -> anyhow::Result<()> {
        let state = Arc::new(AppState { db, http_client });

        let services = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http());

        #[cfg(debug_assertions)]
        let services = services.layer(CorsLayer::permissive());

        let app = Router::new()
            .route("/", get(routes::index))
            .nest("/healthcheck", api::healthcheck())
            .nest("/doctors", api::doctors())
            .nest("/rating", api::rating())
            .nest("/specialties", api::specialties())
            .nest("/notify", api::notification())
            .route("/login", post(routes::authentication::login))
            .route("/register", post(routes::authentication::register))
            .layer(map_request_with_state(state.clone(), token_map))
            .with_state(state)
            .layer(services);


        let addr = SocketAddr::from(([0, 0, 0, 0], 4000));

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
    }

    pub fn doctors() -> Router<Arc<AppState>> {
        Router::new()
            .route("/getInfo", get(routes::doctors::get_info))
    }

    pub fn rating() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::rating::post))
    }

    pub fn specialties() -> Router<Arc<AppState>> {
        Router::new()
            .route("/listAll", post(routes::specialties::list_all))
    }

    pub fn notification() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::notification::post))
    }
}

