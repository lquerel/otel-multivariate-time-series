use otel_multivariate_time_series::multivariate_ts_gen::MultivariateDataPoint;
use otel_multivariate_time_series::opentelemetry::proto::metrics::v1::{ResourceMetrics, InstrumentationLibraryMetrics, Metric, Gauge, NumberDataPoint};
use otel_multivariate_time_series::opentelemetry::proto::resource::v1::Resource;
use otel_multivariate_time_series::opentelemetry::proto::common::v1::{KeyValue, AnyValue, InstrumentationLibrary};
use otel_multivariate_time_series::opentelemetry::proto::common::v1::any_value::Value;
use otel_multivariate_time_series::opentelemetry::proto::metrics::v1::metric::Data;
use otel_multivariate_time_series::opentelemetry::proto::metrics::v1::number_data_point;

use crate::dataset::Dataset;
use crate::profiler::{Profiler, ProfilableProtocol};
use prost::EncodeError;
use prost::Message;
use bytes::Bytes;
use std::error::Error;

struct Test {
    dataset: Dataset<MultivariateDataPoint>,
    resource_metrics: Option<ResourceMetrics>,
}

pub fn profile(profiler: &mut Profiler, dataset: &Dataset<MultivariateDataPoint>, max_iter: usize) -> Result<(), Box<dyn Error>> {
    let mut test = Test {
        dataset: dataset.clone(),
        resource_metrics: None,
    };
    profiler.profile(&mut test, max_iter)
}

impl ProfilableProtocol for Test {
    fn name(&self) -> String {
        "Metrics ref impl".into()
    }

    fn init_batch_size(&mut self, _batch_size: usize) {
    }

    fn dataset_size(&self) -> usize {
        self.dataset.values.len()
    }

    fn create_batch(&mut self, start_at: usize, size: usize) {
        self.resource_metrics = Some(gen_standard_metrics(&self.dataset.values[start_at..start_at+size]));
    }

    fn process(&self) -> String {
        let mut sum = 0;

        let lib_metrics = &self.resource_metrics.as_ref().expect("resource metrics not found").instrumentation_library_metrics[0];
        if let Some(Data::Gauge(gauge)) = &lib_metrics.metrics[0].data {
            for value in &gauge.data_points {
                if let Some(number_data_point::Value::AsInt(value)) = &value.value {
                    sum += *value;
                }
            }
        }
        if let Some(Data::Gauge(gauge)) = &lib_metrics.metrics[1].data {
            for value in &gauge.data_points {
                if let Some(number_data_point::Value::AsInt(value)) = &value.value {
                    sum += *value;
                }
            }
        }

        format!("{}", sum)
    }

    fn serialize(&self) -> Result<Vec<u8>,EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        self.resource_metrics
            .as_ref()
            .expect("resource metrics not found")
            .encode(&mut buf)?;
        Ok(buf)
    }

    fn deserialize(&mut self, buffer: Vec<u8>) {
        self.resource_metrics = Some(ResourceMetrics::decode(Bytes::from(buffer)).unwrap());
    }

    fn clear(&mut self) {
        self.resource_metrics = None;
    }
}

pub fn gen_standard_metrics(time_series: &[MultivariateDataPoint]) -> ResourceMetrics {
    let mut dns_lookup_ms_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        dns_lookup_ms_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.dns_lookup_ms)),
            ..Default::default()
        });
    }

    let mut size_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        size_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.size)),
            ..Default::default()
        });
    }

    let mut content_transfer_ms_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        content_transfer_ms_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.content_transfer_ms)),
            ..Default::default()
        });
    }

    let mut failure_count_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        failure_count_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.failure_count)),
            ..Default::default()
        });
    }

    let mut health_status_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        health_status_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.health_status)),
            ..Default::default()
        });
    }

    let mut server_processing_ms_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        server_processing_ms_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.server_processing_ms)),
            ..Default::default()
        });
    }

    let mut tcp_connection_ms_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        tcp_connection_ms_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.tcp_connection_ms)),
            ..Default::default()
        });
    }

    let mut tls_handshake_ms_points = vec![];
    for data_point in time_series {
        let mut attributes = vec![];
        attributes.push(KeyValue { key: "method".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.method.clone())) }) });
        attributes.push(KeyValue { key: "url".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.url.clone())) }) });
        attributes.push(KeyValue { key: "dns_lookup_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.dns_lookup_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "remote_address".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.remote_address.clone())) }) });
        attributes.push(KeyValue { key: "server_processing_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.server_processing_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "source".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.source.clone())) }) });
        attributes.push(KeyValue { key: "tcp_connection_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tcp_connection_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "tls_handshake_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.tls_handshake_ms_label_class.clone())) }) });
        attributes.push(KeyValue { key: "content_transfer_ms_label_class".into(), value: Some(AnyValue { value: Some(Value::StringValue(data_point.evt.tags.content_transfer_ms_label_class.clone())) }) });

        tls_handshake_ms_points.push(NumberDataPoint {
            attributes,
            start_time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            time_unix_nano: data_point.ts.timestamp_nanos() as u64,
            value: Some(number_data_point::Value::AsInt(data_point.evt.fields.tls_handshake_ms)),
            ..Default::default()
        });
    }

    ResourceMetrics {
        resource: Some(Resource {
            attributes: vec![
                KeyValue { key: "key_1".into(), value: Some(AnyValue { value: Some(Value::StringValue("val1".into())) }) },
                KeyValue { key: "key_2".into(), value: Some(AnyValue { value: Some(Value::StringValue("val2".into())) }) },
                KeyValue { key: "key_3".into(), value: Some(AnyValue { value: Some(Value::StringValue("val3".into())) }) },
            ],
            dropped_attributes_count: 0,
        }),
        instrumentation_library_metrics: vec![
            InstrumentationLibraryMetrics {
                instrumentation_library: Some(InstrumentationLibrary { name: "rust-std".into(), version: "1.0".into() }),
                metrics: vec![
                    Metric {
                        name: "tls_handshake_ms".into(),
                        description: "".into(),
                        unit: "ms".into(),
                        data: Some(Data::Gauge(Gauge { data_points: tls_handshake_ms_points })),
                    },
                    Metric {
                        name: "dns_lookup_ms".into(),
                        description: "".into(),
                        unit: "ms".into(),
                        data: Some(Data::Gauge(Gauge { data_points: dns_lookup_ms_points })),
                    },
                    Metric {
                        name: "size".into(),
                        description: "".into(),
                        unit: "By".into(),
                        data: Some(Data::Gauge(Gauge { data_points: size_points })),
                    },
                    Metric {
                        name: "content_transfer_ms".into(),
                        description: "".into(),
                        unit: "ms".into(),
                        data: Some(Data::Gauge(Gauge { data_points: content_transfer_ms_points })),
                    },
                    Metric {
                        name: "failure_count".into(),
                        description: "".into(),
                        unit: "".into(),
                        data: Some(Data::Gauge(Gauge { data_points: failure_count_points })),
                    },
                    Metric {
                        name: "health_status".into(),
                        description: "".into(),
                        unit: "".into(),
                        data: Some(Data::Gauge(Gauge { data_points: health_status_points })),
                    },
                    Metric {
                        name: "server_processing_ms".into(),
                        description: "".into(),
                        unit: "".into(),
                        data: Some(Data::Gauge(Gauge { data_points: server_processing_ms_points })),
                    },
                    Metric {
                        name: "tcp_connection_ms".into(),
                        description: "".into(),
                        unit: "ms".into(),
                        data: Some(Data::Gauge(Gauge { data_points: tcp_connection_ms_points })),
                    },
                ],
                multivariate_metrics: vec![],
                schema_url: "tbd".into(),
            }
        ],
        schema_url: "tbd".into(),
    }
}