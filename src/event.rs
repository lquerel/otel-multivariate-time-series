use valuable::Valuable;
use std::marker::PhantomData;
use crate::opentelemetry::proto::events::v1::{ResourceEvents, InstrumentationLibraryEvents, BatchEvent, Column, column, StringColumn, Int64Column};
use crate::opentelemetry::proto::resource::v1::Resource;
use crate::opentelemetry::proto::common::v1::InstrumentationLibrary;

#[derive(Debug, Clone)]
pub struct BatchPolicy {
    max_size: usize,
    max_delay: chrono::Duration,
}

#[derive(Debug)]
pub struct MetricBatchHandler<T> {
    schema_url: String,
    batch_policy: BatchPolicy,
    phantom_data: PhantomData<T>,
    resource_events: ResourceEvents,
}

#[derive(Debug)]
pub struct EventCollector {
    default_batch_policy: BatchPolicy,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error (error: {0})")]
    IoError(#[from] std::io::Error),
}

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

impl BatchPolicy {
    pub fn new(max_size: usize, max_delay: chrono::Duration) -> Self {
        BatchPolicy { max_size, max_delay }
    }
}

impl EventCollector {
    pub fn new(batch_policy: BatchPolicy) -> Self {
        EventCollector {
            default_batch_policy: batch_policy,
        }
    }

    pub fn metric_handler(&self, schema_url: &str) -> MetricBatchHandler<HttpTransaction> {
        MetricBatchHandler {
            schema_url: schema_url.into(),
            batch_policy: self.default_batch_policy.clone(),
            phantom_data: PhantomData::default(),
            resource_events: ResourceEvents {
                resource: Some(Resource {
                    attributes: vec![],
                    dropped_attributes_count: 0
                }),
                instrumentation_library_events: vec![
                    InstrumentationLibraryEvents {
                        instrumentation_library: Some(InstrumentationLibrary { name: "rust-std".into(), version: "1.0".into() }),
                        batches: vec![
                            BatchEvent {
                                schema_url: schema_url.into(),
                                size: 0,
                                start_time_unix_nano_column: vec![],
                                end_time_unix_nano_column: vec![],
                                columns: vec![
                                    Column { r#type: Some(column::Type::StringValues(StringColumn {
                                        name: "host".to_string(),
                                        logical_type: 0,
                                        description: "".to_string(),
                                        values: vec![],
                                        validity_bitmap: vec![]
                                    }))
                                    },
                                    Column { r#type: Some(column::Type::I64Values(Int64Column {
                                        name: "port".to_string(),
                                        logical_type: 0,
                                        description: "".to_string(),
                                        unit: "".to_string(),
                                        aggregation_temporality: 0,
                                        is_monotonic: false,
                                        values: vec![],
                                        validity_bitmap: vec![]
                                    }))
                                    },
                                    Column { r#type: Some(column::Type::StringValues(StringColumn {
                                        name: "path".to_string(),
                                        logical_type: 0,
                                        description: "".to_string(),
                                        values: vec![],
                                        validity_bitmap: vec![]
                                    }))
                                    },
                                    Column { r#type: Some(column::Type::StringValues(StringColumn {
                                        name: "query".to_string(),
                                        logical_type: 0,
                                        description: "".to_string(),
                                        values: vec![],
                                        validity_bitmap: vec![]
                                    }))
                                    },
                                    Column { r#type: Some(column::Type::StringValues(StringColumn {
                                        name: "method".to_string(),
                                        logical_type: 0,
                                        description: "".to_string(),
                                        values: vec![],
                                        validity_bitmap: vec![]
                                    }))
                                    },
                                    Column { r#type: Some(column::Type::I64Values(Int64Column {
                                        name: "http_code".to_string(),
                                        logical_type: 0,
                                        description: "".to_string(),
                                        unit: "".to_string(),
                                        aggregation_temporality: 0,
                                        is_monotonic: false,
                                        values: vec![],
                                        validity_bitmap: vec![]
                                    }))
                                    },
                                ],
                                auxiliary_entities: vec![]
                            }
                        ],
                        dropped_events_count: 0
                    }
                ],
                schema_url: schema_url.into(),
            }
        }
    }
}

impl MetricBatchHandler<HttpTransaction> {
    pub fn record(&mut self, event: HttpTransaction) -> Result<(), Error> {
        println!("{:?}",event);
        Ok(())
    }
}


