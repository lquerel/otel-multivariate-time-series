use otel_multivariate_time_series::multivariate_ts_gen::MultivariateDataPoint;

use crate::dataset::Dataset;
use crate::profiler::{Profiler, ProfilableProtocol};
use prost::EncodeError;
use prost::Message;
use bytes::Bytes;
use otel_multivariate_time_series::opentelemetry::proto::events::v1::{ResourceEvents, InstrumentationLibraryEvents, BatchEvent, StringColumn, Int64Column};
use otel_multivariate_time_series::opentelemetry::proto::resource::v1::Resource;
use otel_multivariate_time_series::opentelemetry::proto::common::v1::{KeyValue, AnyValue, InstrumentationLibrary};
use otel_multivariate_time_series::opentelemetry::proto::common::v1::any_value::Value;
use std::error::Error;

struct Test {
    dataset: Dataset<MultivariateDataPoint>,
    resource_events: Option<ResourceEvents>,
}

pub fn profile(profiler: &mut Profiler, dataset: &Dataset<MultivariateDataPoint>, max_iter: usize) -> Result<(), Box<dyn Error>> {
    let mut test = Test {
        dataset: dataset.clone(),
        resource_events: None,
    };
    profiler.profile(&mut test, max_iter)
}

impl ProfilableProtocol for Test {
    fn name(&self) -> String {
        "Metrics event based".into()
    }

    fn init_batch_size(&mut self, _batch_size: usize) {}

    fn dataset_size(&self) -> usize {
        self.dataset.values.len()
    }

    fn create_batch(&mut self, start_at: usize, size: usize) {
        self.resource_events = Some(gen_columnar_metrics(&self.dataset.values[start_at..start_at + size]));
    }

    fn process(&self) -> String {
        let mut sum = 0;

        let batch = &self.resource_events.as_ref().expect("resource events not found").instrumentation_library_events[0].batches[0];
        let tls_handshake_ms = &batch.i64_values[0];
        sum += tls_handshake_ms.values.iter().sum::<i64>() as i32;
        let dns_lookup_ms = &batch.i64_values[1];
        sum += dns_lookup_ms.values.iter().sum::<i64>() as i32;

        format!("{}", sum)
    }

    fn serialize(&self) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        self.resource_events
            .as_ref()
            .expect("resource events not found")
            .encode(&mut buf)?;
        Ok(buf)
    }

    fn deserialize(&mut self, buffer: Vec<u8>) {
        self.resource_events = Some(ResourceEvents::decode(Bytes::from(buffer)).unwrap());
    }

    fn clear(&mut self) {
        self.resource_events = None;
    }
}

pub fn gen_columnar_metrics(time_series: &[MultivariateDataPoint]) -> ResourceEvents {
    ResourceEvents {
        resource: Some(Resource {
            attributes: vec![
                KeyValue { key: "key_1".into(), value: Some(AnyValue { value: Some(Value::StringValue("val1".into())) }) },
                KeyValue { key: "key_2".into(), value: Some(AnyValue { value: Some(Value::StringValue("val2".into())) }) },
                KeyValue { key: "key_3".into(), value: Some(AnyValue { value: Some(Value::StringValue("val3".into())) }) },
            ],
            dropped_attributes_count: 0,
        }),
        instrumentation_library_events: vec![
            InstrumentationLibraryEvents {
                instrumentation_library: Some(InstrumentationLibrary { name: "otel-rust".into(), version: "1.0".into() }),
                batches: vec![
                    BatchEvent {
                        schema_url: "tbd".into(),
                        size: 0,
                        start_time_unix_nano_column: time_series.iter().map(|p| p.ts.timestamp_nanos() as u64).collect(),
                        end_time_unix_nano_column: time_series.iter().map(|p| p.ts.timestamp_nanos() as u64).collect(),
                        i64_values: vec![
                            Int64Column {
                                name: "tls_handshake_ms".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "ms".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.tls_handshake_ms).collect(),
                                validity_bitmap: vec![],
                            },
                            Int64Column {
                                name: "dns_lookup_ms".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "ms".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.dns_lookup_ms).collect(),
                                validity_bitmap: vec![],
                            },
                            Int64Column {
                                name: "server_processing_ms".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "ms".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.server_processing_ms).collect(),
                                validity_bitmap: vec![],
                            },
                            Int64Column {
                                name: "tcp_connection_ms".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "ms".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.tcp_connection_ms).collect(),
                                validity_bitmap: vec![],
                            },
                            Int64Column {
                                name: "content_transfer_ms".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "ms".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.content_transfer_ms).collect(),
                                validity_bitmap: vec![],
                            },
                            Int64Column {
                                name: "health_status".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.health_status).collect(),
                                validity_bitmap: vec![],
                            },
                            Int64Column {
                                name: "failure_count".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.failure_count).collect(),
                                validity_bitmap: vec![],
                            },
                            Int64Column {
                                name: "size".into(),
                                logical_type: 1,    // Gauge
                                description: "".into(),
                                unit: "".into(),
                                aggregation_temporality: 0,
                                is_monotonic: false,
                                values: time_series.iter().map(|p| p.evt.fields.size).collect(),
                                validity_bitmap: vec![],
                            },
                        ],
                        f64_values: vec![],
                        string_values: vec![
                            StringColumn {
                                name: "method".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.method.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "dns_lookup_ms_label_class".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.dns_lookup_ms_label_class.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "source".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.source.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "url".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.url.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "tls_handshake_ms_label_class".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.tls_handshake_ms_label_class.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "remote_address".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.remote_address.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "content_transfer_ms_label_class".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.content_transfer_ms_label_class.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "server_processing_ms_label_class".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.server_processing_ms_label_class.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                            StringColumn {
                                name: "tcp_connection_ms_label_class".into(),
                                logical_type: 0,
                                description: "".into(),
                                values: time_series.iter().map(|p| p.evt.tags.tcp_connection_ms_label_class.clone()).collect(),
                                validity_bitmap: vec![],
                            },
                        ],
                        bool_values: vec![],
                        bytes_values: vec![],
                        i64_summary_values: vec![],
                        f64_summary_values: vec![],
                        auxiliary_entities: vec![],
                    }
                ],
                dropped_events_count: 0,
            }
        ],
        schema_url: "tbd".into(),
    }
}
