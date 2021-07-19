use otel_multivariate_time_series::event::{EventCollector, BatchPolicy, EventBatchHandler};
use otel_multivariate_time_series::error::Error;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use prost::{Message, EncodeError};

use otel_multivariate_time_series::native_trace::NativeTraceHandler;
use otel_multivariate_time_series::opentelemetry::proto::trace::v1::{Span, Status};

mod json_trace;
mod profiler;
mod dataset;

use json_trace::JsonTrace;
use otel_multivariate_time_series::opentelemetry::proto::common::v1::{KeyValue, AnyValue, any_value};
use crate::dataset::Dataset;
use crate::profiler::ProfilableProtocol;
use crate::profiler::Profiler;

struct Test {
    dataset: Dataset,
    trace_handler: NativeTraceHandler,
}

fn main() -> Result<(), Error> {
    let mut test = Test::new("trace_samples.json"  );
    let mut profiler = Profiler::new();
    profiler.profile(&mut test, vec![10, 100, 1000], 1);
    dbg!(profiler);

    Ok(())
}

impl Test {
    pub fn new(trace_file: &str) -> Self {
        Self {
            dataset: Dataset::new(trace_file),
            trace_handler: NativeTraceHandler::new(),
        }
    }
}

pub fn json_trace_to_span(json_trace: JsonTrace) -> Span {
    let mut attributes = vec![];

    for (key, value) in json_trace.evt.attributes.unwrap_or_default() {
        if let Some(value) = value {
            attributes.push(KeyValue { key, value: Some(AnyValue {value: Some(any_value::Value::StringValue(value))}) });
        }
    }

    Span {
        trace_id: json_trace.evt.trace_id.into_bytes(),
        span_id: json_trace.evt.span_id.into_bytes(),
        trace_state: json_trace.evt.trace_state.unwrap_or_default(),
        parent_span_id: json_trace.evt.parent_span_id.unwrap_or_default().into_bytes(),
        name: json_trace.evt.name,
        kind: json_trace.evt.kind.unwrap_or_default() as i32,
        start_time_unix_nano: json_trace.evt.start_time_utc.timestamp_nanos() as u64,
        end_time_unix_nano: json_trace.evt.end_time_utc.timestamp_nanos() as u64,
        attributes,
        dropped_attributes_count: 0,
        events: vec![],
        dropped_events_count: 0,
        links: vec![],
        dropped_links_count: 0,
        status: Some(Status {
            deprecated_code: 0,
            message: json_trace.evt.status.message.unwrap_or_default(),
            code: json_trace.evt.status.code.unwrap_or_default() as i32,
        })
    }
}

impl ProfilableProtocol for Test {
    fn name(&self) -> String {
        "otel_trace_v1".into()
    }

    fn dataset_size(&self) -> usize {
        self.dataset.traces.len()
    }

    fn create_batch(&mut self, start_at: usize, size: usize) {
        for json_trace in self.dataset.traces[start_at..start_at+size].to_vec() {
            self.trace_handler.record(json_trace_to_span(json_trace));
        }
    }

    fn serialize(&self) -> Result<Vec<u8>,EncodeError> {
        self.trace_handler.serialize()
    }

    fn deserialize(&mut self, buffer: Vec<u8>) {
        self.trace_handler.deserialize(buffer);
    }

    fn clear(&mut self) {
        self.trace_handler.clear();
    }
}
