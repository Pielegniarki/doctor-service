use std::{net::SocketAddr, sync::Arc};

use axum::{
    routing::{get, post},
    Router, middleware::map_request_with_state, http::{Request, HeaderMap, HeaderValue}, extract::State,
};

use crate::db::DB;
use crate::http_client::HttpClient;

mod routes;

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
            .nest("/rating", api::rating())
            .nest("/notify", api::notification())
            .route("/login", post(routes::authentication::login))
            .route("/register", post(routes::authentication::register))
            .layer(map_request_with_state(state.clone(), map_id))
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
    }

    pub fn doctors() -> Router<Arc<AppState>> {
        Router::new()
            .route("/getInfo", get(routes::doctors::get_info))
    }

    pub fn rating() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::rating::post))
    }

    pub fn notification() -> Router<Arc<AppState>> {
        Router::new()
            .route("/", post(routes::notification::post))
    }
}

async fn map_id<Body>(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,

    mut request: Request<Body>
) -> Request<Body> {
    if let Some(token) = headers.get("plg-token") {
        let client = &state.http_client;
        let token_str = token.to_str().expect("Invalid ASCII in plg-token header");

        if let Ok(Some(db_id)) = client.get_id(token_str).await {
            if let Ok(hv_id) = HeaderValue::try_from(db_id) {
                request.headers_mut().append("plg-id", hv_id);
            }
        }
    }

    request
}
