use std::time::{Instant};
use prost::EncodeError;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Write, LineWriter};
use std::error::Error;
use std::collections::HashMap;
use comfy_table::{Table, Cell, Color, Attribute, ContentArrangement};
use std::fmt::{Display, Formatter};
use comfy_table::presets::UTF8_FULL;
use std::fs::File;

// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;
//
// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

pub trait ProfilableProtocol {
    fn name(&self) -> String;
    fn init_batch_size(&mut self, batch_size: usize);
    fn dataset_size(&self) -> usize;
    fn create_batch(&mut self, start_at: usize, size: usize);
    fn process(&self) -> String { "".into() }
    fn serialize(&self) -> Result<Vec<u8>, EncodeError>;
    fn deserialize(&mut self, buffer: Vec<u8>);
    fn clear(&mut self);
}

#[derive(Debug)]
pub struct Profiler {
    batch_sizes: Vec<usize>,
    benchmarks: Vec<ProfilerResult>,
}

#[derive(Debug)]
pub struct ProfilerResult {
    bench_name: String,
    summaries: Vec<BatchSummary>,
}

#[derive(Debug, Clone)]
struct Metric {
    values: Vec<f64>,
}

#[derive(Debug, Clone)]
struct BatchSummary {
    batch_size: usize,
    uncompressed_size_byte: Summary,
    compressed_size_byte: Summary,
    batch_creation_sec: Summary,
    processing_sec: Summary,
    serialization_sec: Summary,
    deserialization_sec: Summary,
    total_time_sec: Summary,
    processing_results: Vec<String>,
}

#[derive(Debug, Clone)]
struct Summary {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    pub values: Vec<f64>,
}

impl Display for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("min={}, max={}, mean={}, stddev={}, p50={}, p90={}, p99={}", self.min, self.max, self.mean, self.stddev, self.p50, self.p90, self.p99))
    }
}

impl Metric {
    pub fn new() -> Self {
        Self {
            values: vec![]
        }
    }

    pub fn record(&mut self, value: f64) {
        self.values.push(value);
    }

    pub fn compute_summary(&mut self) -> Summary {
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut sum = 0f64;

        for value in &self.values {
            min = min.min(*value);
            max = max.max(*value);
            sum += *value;
        }

        self.local_sort();

        let mean = sum / self.values.len() as f64;

        Summary {
            min,
            max,
            mean,
            stddev: self.std_dev(mean),
            p50: self.percentile(50f64),
            p90: self.percentile(90f64),
            p95: self.percentile(95f64),
            p99: self.percentile(99f64),
            values: self.values.clone(),
        }
    }

    fn percentile(&self, pct: f64) -> f64 {
        assert!(!self.values.is_empty());
        if self.values.len() == 1 {
            return self.values[0];
        }
        let zero: f64 = 0.0;
        assert!(zero <= pct);
        let hundred = 100_f64;
        assert!(pct <= hundred);
        if pct == hundred {
            return self.values[self.values.len() - 1];
        }
        let length = (self.values.len() - 1) as f64;
        let rank = (pct / hundred) * length;
        let lrank = rank.floor();
        let d = rank - lrank;
        let n = lrank as usize;
        let lo = self.values[n];
        let hi = self.values[n + 1];
        lo + (hi - lo) * d
    }

    fn local_sort(&mut self) {
        self.values.sort_by(|x: &f64, y: &f64| x.partial_cmp(y).unwrap());
    }

    fn var(&self, mean: f64) -> f64 {
        if self.values.len() < 2 {
            0.0
        } else {
            let mut v: f64 = 0.0;
            for s in &self.values {
                let x = *s - mean;
                v += x * x;
            }
            let denom = (self.values.len() - 1) as f64;
            v / denom
        }
    }

    fn std_dev(&self, mean: f64) -> f64 {
        self.var(mean).sqrt()
    }
}

impl Profiler {
    pub fn new(batch_sizes: Vec<usize>) -> Self {
        Self { benchmarks: vec![], batch_sizes }
    }

    pub fn profile(&mut self, otel_impl: &mut impl ProfilableProtocol, max_iter: usize) -> Result<(), Box<dyn Error>> {
        self.benchmarks.push(ProfilerResult { bench_name: otel_impl.name(), summaries: vec![] });

        for batch_size in self.batch_sizes.iter() {
            println!("Profiling '{}' (batch-size={})", otel_impl.name(), *batch_size);

            let mut uncompressed_size = Metric::new();
            let mut compressed_size = Metric::new();
            let mut batch_creation = Metric::new();
            let mut processing = Metric::new();
            let mut serialization = Metric::new();
            let mut deserialization = Metric::new();
            let mut total_time = Metric::new();
            let mut processing_results = vec![];

            otel_impl.init_batch_size(*batch_size);

            for _ in 0..max_iter {
                let max_batch_count = otel_impl.dataset_size() / *batch_size;
                let mut start_at = 0;
                for _ in 0..max_batch_count {
                    // Batch creation
                    // jemalloc_ctl::epoch().unwrap();
                    let start = Instant::now();
                    otel_impl.create_batch(start_at, *batch_size);
                    let after_batch_creation = Instant::now();
                    // let allocated = jemalloc_ctl::stats::allocated().unwrap();
                    // let resident = jemalloc_ctl::stats::resident().unwrap();
                    // println!("{} bytes allocated/{} bytes resident", allocated, resident);

                    // Process
                    let result = otel_impl.process();
                    let after_processing = Instant::now();
                    processing_results.push(result);

                    // Serialization
                    let buffer = otel_impl.serialize()?;
                    uncompressed_size.record(buffer.len() as f64);
                    let after_serialization = Instant::now();

                    // Compression
                    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                    e.write_all(&buffer)?;
                    compressed_size.record(e.finish().unwrap().len() as f64);
                    let after_compression = Instant::now();

                    // Deserialization
                    otel_impl.deserialize(buffer);
                    let after_deserialization = Instant::now();
                    otel_impl.clear();

                    start_at += *batch_size;

                    batch_creation.record((after_batch_creation - start).as_secs_f64());
                    processing.record((after_processing - after_batch_creation).as_secs_f64());
                    serialization.record((after_serialization - after_processing).as_secs_f64());
                    deserialization.record((after_deserialization - after_compression).as_secs_f64());

                    total_time.record(
                        (after_batch_creation - start).as_secs_f64()
                            + (after_serialization - after_processing).as_secs_f64()
                            + (after_deserialization - after_compression).as_secs_f64()
                    );
                }
                otel_impl.clear();
            }

            self.benchmarks
                .last_mut()
                .expect("Profiling result not found")
                .summaries
                .push(BatchSummary {
                    batch_size: *batch_size,
                    uncompressed_size_byte: uncompressed_size.compute_summary(),
                    compressed_size_byte: compressed_size.compute_summary(),
                    batch_creation_sec: batch_creation.compute_summary(),
                    processing_sec: processing.compute_summary(),
                    serialization_sec: serialization.compute_summary(),
                    deserialization_sec: deserialization.compute_summary(),
                    total_time_sec: total_time.compute_summary(),
                    processing_results,
                });
        }
        Ok(())
    }

    pub fn check_processing_results(&self) {
        for batch_idx in 0..self.batch_sizes.len() {
            if self.benchmarks.is_empty() {
                continue;
            }

            let mut ref_processing_results = None;
            for result in self.benchmarks.iter() {
                if let Some(ref_processing_results) = ref_processing_results.as_ref() {
                    if *ref_processing_results != result.summaries[batch_idx].processing_results {
                        panic!("Processing results not consistent across the different implementations");
                    }
                } else {
                    ref_processing_results = Some(result.summaries[batch_idx].processing_results.clone());
                }
            }
        }
    }

    pub fn print_results(&self) {
        let mut headers = vec!["Steps".to_string()];
        self.benchmarks.iter().for_each(|r| headers.push(format!("{} (p99)", r.bench_name)));

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        table.set_header(headers);

        let mut values = HashMap::new();
        for result in self.benchmarks.iter() {
            for batch_summary in &result.summaries {
                let key = format!("{}:{}:{}", result.bench_name, batch_summary.batch_size, "compressed_size_byte");
                values.insert(key, batch_summary.compressed_size_byte.clone());
                let key = format!("{}:{}:{}", result.bench_name, batch_summary.batch_size, "uncompressed_size_byte");
                values.insert(key, batch_summary.uncompressed_size_byte.clone());
                let key = format!("{}:{}:{}", result.bench_name, batch_summary.batch_size, "batch_creation_sec");
                values.insert(key, batch_summary.batch_creation_sec.clone());
                let key = format!("{}:{}:{}", result.bench_name, batch_summary.batch_size, "processing_sec");
                values.insert(key, batch_summary.processing_sec.clone());
                let key = format!("{}:{}:{}", result.bench_name, batch_summary.batch_size, "serialization_sec");
                values.insert(key, batch_summary.serialization_sec.clone());
                let key = format!("{}:{}:{}", result.bench_name, batch_summary.batch_size, "deserialization_sec");
                values.insert(key, batch_summary.deserialization_sec.clone());
                let key = format!("{}:{}:{}", result.bench_name, batch_summary.batch_size, "total_time_sec");
                values.insert(key, batch_summary.total_time_sec.clone());
            }
        }

        self.add_section("Batch creation (ms)", "batch_creation_sec", &mut table, &mut values, |value| value * 1000.0);
        self.add_section("Batch processing (ms)", "processing_sec", &mut table, &mut values, |value| value * 1000.0);
        self.add_section("Serialization (ms)", "serialization_sec", &mut table, &mut values, |value| value * 1000.0);
        self.add_section("Compressed size (bytes)", "compressed_size_byte", &mut table, &mut values, |value| value);
        self.add_section("Uncompressed size (bytes)", "uncompressed_size_byte", &mut table, &mut values, |value| value);
        self.add_section("Deserialisation (ms)", "deserialization_sec", &mut table, &mut values, |value| value * 1000.0);
        self.add_section("Total time (ms)", "total_time_sec", &mut table, &mut values, |value| value * 1000.0);

        println!("{}", table);
    }

    fn add_section(&self, label: &str, step: &str, table: &mut Table, values: &mut HashMap<String, Summary>, transform: fn(f64) -> f64) {
        table.add_row(vec![
            Cell::new(label).fg(Color::Green).add_attribute(Attribute::Bold),
            Cell::new(""),
            Cell::new(""),
        ]);

        for batch_size in &self.batch_sizes {
            let mut row = vec![format!("batch_size: {}", *batch_size)];
            let mut ref_impl_name = None;
            for result in self.benchmarks.iter() {
                let key = format!("{}:{}:{}", result.bench_name, *batch_size, step);
                let mut improvement = "".into();

                if let Some(ref_impl_name) = &ref_impl_name {
                    let ref_key = format!("{}:{}:{}", ref_impl_name, *batch_size, step);
                    improvement = format!(" (x{:.2})", values[&ref_key].p99 / values[&key].p99);
                } else {
                    ref_impl_name = Some(result.bench_name.clone());
                }

                let value = transform(values[&key].p99);
                if value >= 1.0 {
                    row.push(format!("{:.5}{}", value, improvement));
                } else {
                    row.push(format!("{:.10}{}", value, improvement));
                }
            }
            table.add_row(row);
        }
    }

    pub fn to_csv(&self, file_prefix: &str) -> Result<(), Box<dyn Error>> {
        self.write_csv_values(
            &mut LineWriter::new(File::create(format!("{}_batch_creation_ms.csv", file_prefix))?),
            |summary| &summary.batch_creation_sec,
            |value| value * 1000.0,
        )?;
        self.write_csv_values(
            &mut LineWriter::new(File::create(format!("{}_processing_ms.csv", file_prefix))?),
            |summary| &summary.processing_sec,
            |value| value * 1000.0,
        )?;
        self.write_csv_values(
            &mut LineWriter::new(File::create(format!("{}_serialization_ms.csv", file_prefix))?),
            |summary| &summary.serialization_sec,
            |value| value * 1000.0,
        )?;
        self.write_csv_values(
            &mut LineWriter::new(File::create(format!("{}_compressed_size_byte.csv", file_prefix))?),
            |summary| &summary.compressed_size_byte,
            |value| value,
        )?;
        self.write_csv_values(
            &mut LineWriter::new(File::create(format!("{}_uncompressed_size_byte.csv", file_prefix))?),
            |summary| &summary.uncompressed_size_byte,
            |value| value,
        )?;
        self.write_csv_values(
            &mut LineWriter::new(File::create(format!("{}_deserialization_ms.csv", file_prefix))?),
            |summary| &summary.deserialization_sec,
            |value| value * 1000.0,
        )?;
        self.write_csv_values(
            &mut LineWriter::new(File::create(format!("{}_total_time_ms.csv", file_prefix))?),
            |summary| &summary.total_time_sec,
            |value| value * 1000.0,
        )?;

        Ok(())
    }

    fn write_csv_values(&self, file: &mut LineWriter<File>,
                        summary_sel: fn(&BatchSummary) -> &Summary,
                        transform: fn(f64) -> f64) -> Result<(), Box<dyn Error>> {
        file.write_all(b"batch_size,iteration")?;
        for result in self.benchmarks.iter() {
            file.write_all(format!(",{}", result.bench_name).as_bytes())?;
        }
        file.write_all(b"\n")?;

        for (batch_idx, batch_size) in self.batch_sizes.iter().enumerate() {
            if self.benchmarks.is_empty() {
                continue;
            }

            let num_samples = summary_sel(&self.benchmarks[0].summaries[batch_idx]).values.len();
            for sample_idx in 0..num_samples {
                let mut line = format!("{},{}", batch_size, sample_idx);
                for result in self.benchmarks.iter() {
                    line.push_str(&format!(",{}", transform(summary_sel(&result.summaries[batch_idx]).values[sample_idx])));
                }
                line.push_str("\n");
                file.write_all(line.as_bytes())?;
            }
        }

        Ok(())
    }

    // ToDo build charts with plotly.rs
}
