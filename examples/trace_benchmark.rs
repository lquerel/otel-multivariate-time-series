use std::error::Error;

mod otel_v1_trace_example;
mod generic_attr_trace_example;
pub mod json_trace;
pub mod profiler;
pub mod dataset;

use crate::profiler::Profiler;
use crate::dataset::Dataset;
use crate::json_trace::JsonTrace;

// RUSTFLAGS="-C target-cpu=native" cargo +nightly run --release --example trace_benchmark
fn main() -> Result<(), Box<dyn Error>> {
    let dataset: Dataset<JsonTrace> = Dataset::new("trace_samples.json");

    let mut profiler = Profiler::new(vec![10, 100, 500, 1000, 5000, 10000]);

    let max_iter = 2;

    otel_v1_trace_example::profile(&mut profiler, &dataset,max_iter);
    generic_attr_trace_example::profile(&mut profiler, &dataset,max_iter);

    profiler.check_processing_results();
    profiler.print_results();
    profiler.export_to_multiple_csv_files("trace");

    Ok(())
}
