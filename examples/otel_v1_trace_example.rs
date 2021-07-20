use otel_multivariate_time_series::event::{EventCollector, BatchPolicy, EventBatchHandler};
use otel_multivariate_time_series::error::Error;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use prost::{Message, EncodeError};

use otel_multivariate_time_series::native_trace::NativeTraceHandler;
use otel_multivariate_time_series::opentelemetry::proto::trace::v1::{Span, Status};

use otel_multivariate_time_series::opentelemetry::proto::common::v1::{KeyValue, AnyValue, any_value};
use crate::dataset::Dataset;
use crate::profiler::{Profiler,ProfilableProtocol};
use crate::json_trace::JsonTrace;

struct Test {
    dataset: Dataset<JsonTrace>,
    trace_handler: NativeTraceHandler,
}

pub fn profile(profiler: &mut Profiler, dataset: &Dataset<JsonTrace>, max_iter: usize) {
    let mut test = Test::new(dataset  );
    profiler.profile(&mut test, max_iter);
}

impl Test {
    pub fn new(dataset: &Dataset<JsonTrace>) -> Self {
        Self {
            dataset: dataset.clone(),
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
        "Trace ref impl".into()
    }

    fn init_batch_size(&mut self, batch_size: usize) {
    }

    fn dataset_size(&self) -> usize {
        self.dataset.values.len()
    }

    fn create_batch(&mut self, start_at: usize, size: usize) {
        for json_trace in self.dataset.values[start_at..start_at+size].to_vec() {
            self.trace_handler.record(json_trace_to_span(json_trace));
        }
    }

    fn process(&self) -> String {
        let mut sum = 0;

        for span in &self.trace_handler.resource_spans.instrumentation_library_spans[0].spans {
            sum += span.kind;
        }
        for span in &self.trace_handler.resource_spans.instrumentation_library_spans[0].spans {
            if let Some(status) = &span.status {
                sum += status.code;
            }
        }

        format!("{}", sum)
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
