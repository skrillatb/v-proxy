use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;

use crate::handlers::proxy_handler;

#[derive(Clone)]
pub struct AppState {
    pub server_addr: SocketAddr,
}

pub fn create_app(server_addr: SocketAddr) -> Router {
    let state = AppState { server_addr };
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/proxy", get(proxy_handler))
        .layer(cors)
        .with_state(state)
}
