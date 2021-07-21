use std::error::Error;

pub mod profiler;
pub mod dataset;
pub mod otel_v1;
pub mod otel_columnar;
pub mod otel_arrow;

use crate::profiler::Profiler;
use crate::dataset::Dataset;
use otel_multivariate_time_series::multivariate_ts_gen::MultivariateDataPoint;

fn main() -> Result<(), Box<dyn Error>> {
    let dataset: Dataset<MultivariateDataPoint> = Dataset::new("multivariate-time-series.json");
    let mut profiler = Profiler::new(vec![10, 100, 500, 1000, 5000, 10000]);

    let max_iter = 2;

    otel_v1::metrics::profile(&mut profiler, &dataset,max_iter)?;
    otel_columnar::metrics::profile(&mut profiler, &dataset, max_iter)?;
    otel_arrow::metrics::profile(&mut profiler, &dataset, max_iter)?;

    profiler.check_processing_results();
    profiler.print_results();
    profiler.to_csv("metrics")?;

    Ok(())
}
