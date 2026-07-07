pub mod handlers;
pub mod middleware;
pub mod router;

use crate::AppState;
use axum::Router;

pub fn router(_state: AppState) -> Router {
    Router::new()
}

pub async fn serve(state: AppState) -> ! {
    let addr = format!(
        "{}:{}",
        state.config.api.bind_address, state.config.api.bind_port
    );

    tracing::info!("API server listening on {addr}");

    let app = router(state);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind API address");

    axum::serve(listener, app)
        .await
        .expect("API server failed");

    unreachable!()
}
