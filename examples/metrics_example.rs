use chrono::Utc;

use http_transaction::HttpTransaction;
use otel_multivariate_time_series::event::{BatchPolicy, Error, EventCollector, EventBatchHandler, OpenTelemetryEvent};
use otel_multivariate_time_series::opentelemetry::proto::events::v1::{Int64Column, StringColumn};

mod http_transaction;

// ToDo Minimize allocations
// ToDo Serialize and test perf
// ToDo Add Option support with validity_bitmap

fn main() -> Result<(), Error>{
    let event_collector = EventCollector::new(BatchPolicy::new(10, chrono::Duration::seconds(10)));
    let mut http_transaction_handler: EventBatchHandler<HttpTransaction> = event_collector.event_handler();

    for i in 0..1000 {
        http_transaction_handler.record(HttpTransaction {
            host: "f5.com".into(),
            port: 443,
            path: format!("/{}.html", i),
            query: "".into(),
            method: "GET".into(),
            http_code: 200,
            dns_latency_ms: 10,
            tls_handshake_ms: 10,
            content_transfer_ms: 100,
            server_processing_ms: 50,
            request_size_bytes: 1000,
            response_size_bytes: 10000 + (i*10),
        })?;
    }

    println!("{:#?}", http_transaction_handler);

    Ok(())
}

