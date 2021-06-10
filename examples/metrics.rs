use valuable::Valuable;
use serde::Serialize;

use otel_multivariate_time_series::event::{EventCollector, BatchPolicy, MetricBatchHandler, HttpTransaction};

fn main() {
    let event_collector = EventCollector::new(BatchPolicy::new(100, chrono::Duration::seconds(10)));
    let mut http_transaction_handler = event_collector.metric_handler("urn:project:ns:name");

    let http_transaction = HttpTransaction {
        host: "f5.com".into(),
        port: 443,
        path: "/".into(),
        query: "".into(),
        method: "GET".into(),
        http_code: 0,
        dns_latency_ms: 0,
        tls_handshake_ms: 0,
        content_transfer_ms: 0,
        server_processing_ms: 0,
        request_size_bytes: 0,
        response_size_bytes: 0
    };

    http_transaction_handler.record(http_transaction);
}
