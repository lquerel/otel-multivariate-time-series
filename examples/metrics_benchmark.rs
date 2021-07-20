use std::error::Error;

pub mod profiler;
pub mod dataset;
pub mod otel_v1;
pub mod otel_event;

use crate::profiler::Profiler;
use crate::dataset::Dataset;
use otel_multivariate_time_series::multivariate_ts_gen::MultivariateDataPoint;

fn main() -> Result<(), Box<dyn Error>> {
    let dataset: Dataset<MultivariateDataPoint> = Dataset::new("multivariate-time-series.json");
    let mut profiler = Profiler::new(vec![10, 100, 500, 1000, 5000, 10000]);

    let max_iter = 2;

    otel_v1::metrics::profile(&mut profiler, &dataset,max_iter)?;
    otel_event::metrics::profile(&mut profiler, &dataset,max_iter)?;

    profiler.print_results();

    Ok(())
}