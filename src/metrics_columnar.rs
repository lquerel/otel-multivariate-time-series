use crate::multivariate_ts_gen::MultivariateDataPoint;
use crate::opentelemetry::proto::resource::v1::Resource;
use crate::opentelemetry::proto::common::v1::{KeyValue, AnyValue, InstrumentationLibrary};
use crate::opentelemetry::proto::common::v1::any_value::Value;
use crate::opentelemetry::proto::metrics::v1::{ResourceMetrics, InstrumentationLibraryMetrics, MultivariateMetric, ColumnarAttribute, ColumnarMetric, ColumnarGauge, ColumnarNumberDataPoint, IntValues, columnar_number_data_point};
use crate::opentelemetry::proto::metrics::v1::columnar_metric::Data;

pub fn gen_columnar_metrics(time_series: &[MultivariateDataPoint]) -> ResourceMetrics {
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
                metrics: vec![],
                multivariate_metrics: vec![
                    MultivariateMetric {
                        attributes: vec![
                            ColumnarAttribute { name: "method".into(), values: time_series.iter().map(|p| p.evt.tags.method.clone()).collect() },
                            ColumnarAttribute { name: "dns_lookup_ms_label_class".into(), values: time_series.iter().map(|p| p.evt.tags.dns_lookup_ms_label_class.clone()).collect() },
                            ColumnarAttribute { name: "source".into(), values: time_series.iter().map(|p| p.evt.tags.source.clone()).collect() },
                            ColumnarAttribute { name: "url".into(), values: time_series.iter().map(|p| p.evt.tags.url.clone()).collect() },
                            ColumnarAttribute { name: "tls_handshake_ms_label_class".into(), values: time_series.iter().map(|p| p.evt.tags.tls_handshake_ms_label_class.clone()).collect() },
                            ColumnarAttribute { name: "remote_address".into(), values: time_series.iter().map(|p| p.evt.tags.remote_address.clone()).collect() },
                            ColumnarAttribute { name: "content_transfer_ms_label_class".into(), values: time_series.iter().map(|p| p.evt.tags.content_transfer_ms_label_class.clone()).collect() },
                            ColumnarAttribute { name: "server_processing_ms_label_class".into(), values: time_series.iter().map(|p| p.evt.tags.server_processing_ms_label_class.clone()).collect() },
                            ColumnarAttribute { name: "tcp_connection_ms_label_class".into(), values: time_series.iter().map(|p| p.evt.tags.tcp_connection_ms_label_class.clone()).collect() },
                        ],
                        time_unix_nano_column: time_series.iter().map(|p| p.ts.timestamp_nanos() as u64).collect(),
                        start_time_unix_nano_column: time_series.iter().map(|p| p.ts.timestamp_nanos() as u64).collect(),
                        metrics: vec![
                            ColumnarMetric {
                                name: "tls_handshake_ms".into(),
                                description: "".into(),
                                unit: "ms".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.tls_handshake_ms).collect() })) }) }))
                            },
                            ColumnarMetric {
                                name: "dns_lookup_ms".into(),
                                description: "".into(),
                                unit: "ms".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.dns_lookup_ms).collect() })) }) }))
                            },
                            ColumnarMetric {
                                name: "server_processing_ms".into(),
                                description: "".into(),
                                unit: "ms".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.server_processing_ms).collect() })) }) }))
                            },
                            ColumnarMetric {
                                name: "tcp_connection_ms".into(),
                                description: "".into(),
                                unit: "ms".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.tcp_connection_ms).collect() })) }) }))
                            },
                            ColumnarMetric {
                                name: "content_transfer_ms".into(),
                                description: "".into(),
                                unit: "ms".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.content_transfer_ms).collect() })) }) }))
                            },
                            ColumnarMetric {
                                name: "health_status".into(),
                                description: "".into(),
                                unit: "".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.health_status).collect() })) }) }))
                            },
                            ColumnarMetric {
                                name: "failure_count".into(),
                                description: "".into(),
                                unit: "".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.failure_count).collect() })) }) }))
                            },
                            ColumnarMetric {
                                name: "size".into(),
                                description: "".into(),
                                unit: "".into(),
                                data: Some(Data::Gauge(ColumnarGauge { data_points: Some(ColumnarNumberDataPoint { value: Some(columnar_number_data_point::Value::AsInts(IntValues { value: time_series.iter().map(|p| p.evt.fields.size).collect() })) }) }))
                            },
                        ],
                    }
                ],
                schema_url: "tbd".into()
            }
        ],
        schema_url: "tbd".into(),
    }
}