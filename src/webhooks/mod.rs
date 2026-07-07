pub mod error;
pub mod middleware;
pub mod rate_limiter;
pub mod router;
pub mod server;
pub mod signature;

pub use error::WebhookError;

use crate::AppState;

pub async fn serve(state: AppState) -> ! {
    server::start(state).await;
    unreachable!()
}
