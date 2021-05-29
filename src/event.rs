use crate::opentelemetry::proto::events::v1::ResourceMetrics;

pub struct BatchPolicy {
    pub max_size: usize,
    pub max_delay_ms: usize,
}

pub struct EventCollector {
}

pub struct BatchHandler {}

impl EventCollector {
    pub fn batch_handler(schema_url: &str, batch_policy: BatchPolicy) -> BatchHandler {}
}

impl BatchHandler {

}