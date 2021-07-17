use otel_multivariate_time_series::event::{EventCollector, BatchPolicy, EventBatchHandler};
use otel_multivariate_time_series::error::Error;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;

mod json_trace;

use crate::json_trace::JsonTrace;

// ToDo Numerical and boolean attributes could be better represented as numerical or boolean columns
// ToDo Multi nested level (e.g. attributes in events and links)

fn main() -> Result<(), Error> {
    let event_collector = EventCollector::new(BatchPolicy::new(100, chrono::Duration::seconds(10)));
    let mut trace_handler: EventBatchHandler<JsonTrace> = event_collector.event_handler();

    let file = File::open(data_file("trace_samples.json")).expect("trace_samples.json not found");
    let reader = BufReader::new(file);

    let mut json_traces: Vec<JsonTrace> = serde_json::from_reader(reader).expect("trace_samples.json not a readable JSON file");

    let mut count = 0;
    for json_trace in json_traces {
        if count == 10 {
            break;
        }
        trace_handler.record(json_trace);
        count+=1;
    }

    let json_value = trace_handler.to_json_value();
    println!("{}", serde_json::to_string_pretty(&json_value).expect("invalid json serialization"));

    Ok(())
}

pub fn data_file(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("data/{}", file_name));
    path
}

