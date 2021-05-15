use crate::multivariate_ts_gen::MultivariateDataPoint;
use crate::opentelemetry::proto::metrics::v1::{ResourceMetrics, InstrumentationLibraryMetrics, Metric, Gauge, NumberDataPoint};
use crate::opentelemetry::proto::resource::v1::Resource;
use crate::opentelemetry::proto::common::v1::{KeyValue, AnyValue, InstrumentationLibrary};
use crate::opentelemetry::proto::common::v1::any_value::Value;
use crate::opentelemetry::proto::metrics::v1::metric::Data;
use crate::opentelemetry::proto::metrics::v1::number_data_point;

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
                    Metric {
                        name: "tls_handshake_ms".into(),
                        description: "".into(),
                        unit: "ms".into(),
                        data: Some(Data::Gauge(Gauge { data_points: tls_handshake_ms_points })),
                    },
                ],
                multivariate_metrics: vec![],
                schema_url: "tbd".into(),
            }
        ],
        schema_url: "tbd".into(),
    }
}