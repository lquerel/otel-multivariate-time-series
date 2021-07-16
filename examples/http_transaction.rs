use chrono::Utc;
use prost::Message;

use otel_multivariate_time_series::event::{BatchPolicy, EventBatchHandler, OpenTelemetryEvent};
use otel_multivariate_time_series::opentelemetry::proto::events::v1::{Int64Column, StringColumn};

#[derive(Debug)]
pub struct HttpTransaction {
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: String,
    pub method: String,
    pub http_code: u16,
    pub dns_latency_ms: u32,
    pub tls_handshake_ms: u32,
    pub content_transfer_ms: u32,
    pub server_processing_ms: u32,
    pub request_size_bytes: u64,
    pub response_size_bytes: u64,
}


impl OpenTelemetryEvent for HttpTransaction {
    fn urn() -> String {
        "urn:project_a:http:transaction".into()
    }

    fn int64_columns(batch_policy: &BatchPolicy) -> Vec<Int64Column> where Self: Sized {
        vec![
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("port", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("http_code", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("dns_latency_ms", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("tls_handshake_ms", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("content_transfer_ms", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("server_processing_ms", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("request_size_bytes", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_int64_column("response_size_bytes", batch_policy),
        ]
    }

    fn string_columns(batch_policy: &BatchPolicy) -> Vec<StringColumn> where Self: Sized {
        vec![
            <HttpTransaction as OpenTelemetryEvent>::new_string_column("host", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_string_column("path", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_string_column("query", batch_policy),
            <HttpTransaction as OpenTelemetryEvent>::new_string_column("method", batch_policy),
        ]
    }

    fn record_into(self, handler: &mut EventBatchHandler<Self>) where Self: Sized {
        if handler.resource_events.instrumentation_library_events[0].batches[0].size == handler.batch_policy.max_size {
            let mut buf: Vec<u8> = Vec::with_capacity(200);

            handler.resource_events.encode(&mut buf).unwrap();
            println!("{}", buf.len());

            let batch = &mut handler.resource_events.instrumentation_library_events[0].batches[0];

            batch.start_time_unix_nano_column.clear();

            batch.string_values[0].values.clear();
            batch.string_values[1].values.clear();
            batch.string_values[2].values.clear();
            batch.string_values[3].values.clear();

            batch.i64_values[0].values.clear();
            batch.i64_values[1].values.clear();
            batch.i64_values[2].values.clear();
            batch.i64_values[3].values.clear();
            batch.i64_values[4].values.clear();
            batch.i64_values[5].values.clear();
            batch.i64_values[6].values.clear();
            batch.i64_values[7].values.clear();

            batch.size = 0;
        }

        let batch = &mut handler.resource_events.instrumentation_library_events[0].batches[0];

        batch.start_time_unix_nano_column.push(Utc::now().timestamp_nanos() as u64);

        batch.string_values[0].values.push(self.host);
        batch.string_values[1].values.push(self.path);
        batch.string_values[2].values.push(self.query);
        batch.string_values[3].values.push(self.method);

        batch.i64_values[0].values.push(self.port as i64);
        batch.i64_values[1].values.push(self.http_code as i64);
        batch.i64_values[2].values.push(self.dns_latency_ms as i64);
        batch.i64_values[3].values.push(self.tls_handshake_ms as i64);
        batch.i64_values[4].values.push(self.content_transfer_ms as i64);
        batch.i64_values[5].values.push(self.server_processing_ms as i64);
        batch.i64_values[6].values.push(self.request_size_bytes as i64);
        batch.i64_values[7].values.push(self.response_size_bytes as i64);

        batch.size += 1;
    }
}
