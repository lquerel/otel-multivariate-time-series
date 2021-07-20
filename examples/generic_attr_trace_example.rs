use otel_multivariate_time_series::event::{EventCollector, BatchPolicy, EventBatchHandler, is_valid_value};
use otel_multivariate_time_series::error::Error;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;
use prost::EncodeError;
use crate::dataset::Dataset;
use crate::profiler::{Profiler,ProfilableProtocol};
use crate::json_trace::JsonTrace;

// ToDo Numerical and boolean attributes could be better represented as numerical or boolean columns
// ToDo Multi nested level (e.g. attributes in events and links)

struct Test {
    dataset: Dataset<JsonTrace>,
    trace_handler: EventBatchHandler<JsonTrace>,
}

pub fn profile(profiler: &mut Profiler, dataset: &Dataset<JsonTrace>, max_iter: usize) {
    let mut test = Test::new(dataset);
    profiler.profile(&mut test, max_iter);
}

impl Test {
    pub fn new(dataset: &Dataset<JsonTrace>) -> Self {
        Self {
            dataset: dataset.clone(),
            trace_handler: EventBatchHandler::new(BatchPolicy::new(1, chrono::Duration::seconds(10)))
        }
    }
}

impl ProfilableProtocol for Test {
    fn name(&self) -> String {
        "Trace event based".into()
    }

    fn init_batch_size(&mut self, batch_size: usize) {
        self.trace_handler = EventBatchHandler::new(BatchPolicy::new(batch_size as u32, chrono::Duration::seconds(10)));
    }

    fn dataset_size(&self) -> usize {
        self.dataset.values.len()
    }

    fn create_batch(&mut self, start_at: usize, size: usize) {
        for json_trace in self.dataset.values[start_at..start_at+size].to_vec() {
            self.trace_handler.record(json_trace);
        }
    }

    fn process(&self) -> String {
        let mut sum = 0;

        let kind = &self.trace_handler.resource_events.instrumentation_library_events[0].batches[0].i64_values[0];
        for value in &kind.values {
            sum += *value;
        }
        let status_code = &self.trace_handler.resource_events.instrumentation_library_events[0].batches[0].i64_values[1];
        for (i, value) in status_code.values.iter().enumerate() {
            if is_valid_value(&status_code.validity_bitmap, i) {
                sum += *value;
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
        self.trace_handler.reset_batch_event();
    }
}
