use axum::{middleware, Router};
use axum::routing::get;

use crate::AppState;


pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            super::middleware::auth,
        ))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}
