use metrics::counter;
use metrics::histogram;

#[derive(Clone)]
pub struct MetricsCollector;

impl MetricsCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn record_webhook_received(&self, provider: &str) {
        counter!("webhooks.received", "provider" => provider.to_owned()).increment(1);
    }

    pub fn record_webhook_processed(&self, provider: &str, status: &str) {
        counter!("webhooks.processed", "provider" => provider.to_owned(), "status" => status.to_owned())
            .increment(1);
    }

    pub fn record_event_dispatched(&self, event_type: &str) {
        counter!("events.dispatched", "event_type" => event_type.to_owned()).increment(1);
    }

    pub fn record_command_executed(&self, command: &str) {
        counter!("commands.executed", "command" => command.to_owned()).increment(1);
    }

    pub fn record_processing_time(&self, provider: &str, duration_secs: f64) {
        histogram!("webhook.processing_time", "provider" => provider.to_owned())
            .record(duration_secs);
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
