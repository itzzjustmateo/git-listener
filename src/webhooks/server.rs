
use crate::AppState;

pub async fn start(state: AppState) -> tokio::task::JoinHandle<()> {
    let addr = format!(
        "{}:{}",
        state.config.webhook.bind_address, state.config.webhook.bind_port
    );

    let app = super::router::router(state);

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("failed to bind webhook address");

        tracing::info!("webhook server listening on {addr}");

        axum::serve(listener, app)
            .await
            .expect("webhook server failed");
    })
}
