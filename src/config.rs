use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers::proxy_handler;

pub fn create_app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/proxy", get(proxy_handler))
        .layer(cors)
}
