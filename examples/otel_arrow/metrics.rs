use otel_multivariate_time_series::multivariate_ts_gen::MultivariateDataPoint;

use crate::dataset::Dataset;
use crate::profiler::{Profiler, ProfilableProtocol};
use prost::EncodeError;
use prost::Message;
use bytes::Bytes;
use otel_multivariate_time_series::opentelemetry::proto::resource::v1::Resource;
use otel_multivariate_time_series::opentelemetry::proto::common::v1::{KeyValue, AnyValue, InstrumentationLibrary};
use otel_multivariate_time_series::opentelemetry::proto::common::v1::any_value::Value;
use std::error::Error;
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;
use arrow::array::{Int64Array, UInt64Array, StringArray};
use otel_multivariate_time_series::opentelemetry::proto::arrow_events::v1::{ResourceEvents, InstrumentationLibraryEvents, BatchEvent};
use arrow::ipc::writer::StreamWriter;
use arrow::ipc::reader::StreamReader;

struct Test {
    dataset: Dataset<MultivariateDataPoint>,
    schema: Arc<Schema>,
    batch: Option<RecordBatch>,
    resource_events: Option<ResourceEvents>,
}

pub fn profile(profiler: &mut Profiler, dataset: &Dataset<MultivariateDataPoint>, max_iter: usize) -> Result<(), Box<dyn Error>> {
    let mut test = Test {
        dataset: dataset.clone(),
        schema: arrow_schema(),
        batch: None,
        resource_events: None,
    };
    profiler.profile(&mut test, max_iter)
}

impl ProfilableProtocol for Test {
    fn name(&self) -> String {
        "arrow".into()
    }

    fn init_batch_size(&mut self, _batch_size: usize) {}

    fn dataset_size(&self) -> usize {
        self.dataset.values.len()
    }

    fn create_batch(&mut self, start_at: usize, size: usize) {
        self.gen_arrow_buffer(start_at, start_at + size).expect("gen_arrow_buffer error");
    }

    fn process(&self) -> String {
        let mut sum = 0i64;

        if let Some(batch) = self.batch.as_ref() {
            for _ in 0..50 {
                sum += batch.column(2)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .expect("tls_handshake_ms column not accessible")
                    .iter()
                    .sum::<Option<i64>>()
                    .unwrap_or(0);

                sum += batch.column(3)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .expect("dns_lookup_ms column not accessible")
                    .iter()
                    .sum::<Option<i64>>()
                    .unwrap_or(0);

                sum += batch.column(4)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .expect("server_processing_ms column not accessible")
                    .iter()
                    .min()
                    .unwrap_or(Some(0))
                    .expect("server_processing_ms column min not computable");
                sum += batch.column(4)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .expect("server_processing_ms column not accessible")
                    .iter()
                    .max()
                    .unwrap_or(Some(0))
                    .expect("server_processing_ms column min not computable");

                sum += batch.column(5)
                    .as_any()
                    .downcast_ref::<Int64Array>()
                    .expect("tcp_connection_ms column not accessible")
                    .iter()
                    .sum::<Option<i64>>()
                    .unwrap_or(0);
            }
        }

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
        let mut reader = StreamReader::try_new(&self.resource_events.as_ref().unwrap().instrumentation_library_events[0].batches[0].arrow_buffer as &[u8]).expect("stream reader error");
        let batch = reader.next();
        self.batch = Some(batch.unwrap().unwrap());
    }

    fn clear(&mut self) {
        self.resource_events = None;
    }
}

impl Test {
    fn gen_arrow_buffer(&mut self, start_at: usize, end_at: usize) -> Result<(), Box<dyn Error>> {
        let time_series = &self.dataset.values[start_at..end_at];
        self.batch = Some(RecordBatch::try_new(self.schema.clone(), vec![
            Arc::new(UInt64Array::from_iter_values(time_series.iter().map(|p| p.ts.timestamp_nanos() as u64))),
            Arc::new(UInt64Array::from_iter_values(time_series.iter().map(|p| p.ts.timestamp_nanos() as u64))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.tls_handshake_ms))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.dns_lookup_ms))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.server_processing_ms))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.tcp_connection_ms))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.content_transfer_ms))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.health_status))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.failure_count))),
            Arc::new(Int64Array::from_iter_values(time_series.iter().map(|p| p.evt.fields.size))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.method))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.dns_lookup_ms_label_class))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.source))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.url))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.tls_handshake_ms_label_class))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.remote_address))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.content_transfer_ms_label_class))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.server_processing_ms_label_class))),
            Arc::new(StringArray::from_iter_values(time_series.iter().map(|p| &p.evt.tags.tcp_connection_ms_label_class))),
        ])?);

        let mut writer = StreamWriter::try_new(Vec::new(), self.schema.as_ref()).expect("invalid arrow stream writer");
        writer.write(self.batch.as_ref().expect("access batch error")).expect("write batch error");
        writer.finish().expect("finish write error");

        self.resource_events = Some(ResourceEvents {
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
                            schema_url: "tbd".to_string(),
                            size: time_series.len() as u32,
                            arrow_buffer: writer.into_inner().expect("into inner error"),
                        }
                    ],
                    dropped_events_count: 0,
                }
            ],
            schema_url: "tbd".into(),
        });

        Ok(())
    }
}

fn arrow_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        // start/end time
        Field::new("start_time_unix_nano", DataType::UInt64, false),
        Field::new("end_time_unix_nano", DataType::UInt64, false),

        // metrics
        Field::new("tls_handshake_ms", DataType::Int64, false),
        Field::new("dns_lookup_ms", DataType::Int64, false),
        Field::new("server_processing_ms", DataType::Int64, false),
        Field::new("tcp_connection_ms", DataType::Int64, false),
        Field::new("content_transfer_ms", DataType::Int64, false),
        Field::new("health_status", DataType::Int64, false),
        Field::new("failure_count", DataType::Int64, false),
        Field::new("size", DataType::Int64, false),

        // dimensions
        Field::new("method", DataType::Utf8, false),
        Field::new("dns_lookup_ms_label_class", DataType::Utf8, false),
        Field::new("source", DataType::Utf8, false),
        Field::new("url", DataType::Utf8, false),
        Field::new("tls_handshake_ms_label_class", DataType::Utf8, false),
        Field::new("remote_address", DataType::Utf8, false),
        Field::new("content_transfer_ms_label_class", DataType::Utf8, false),
        Field::new("server_processing_ms_label_class", DataType::Utf8, false),
        Field::new("tcp_connection_ms_label_class", DataType::Utf8, false),
    ]))
}

