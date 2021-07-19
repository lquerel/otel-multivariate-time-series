use std::time::{Instant, Duration};
use prost::EncodeError;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use std::error::Error;
use std::collections::HashMap;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub trait ProfilableProtocol {
    fn name(&self) -> String;
    fn dataset_size(&self) -> usize;
    fn create_batch(&mut self, start_at: usize, size: usize);
    fn serialize(&self) -> Result<Vec<u8>,EncodeError>;
    fn deserialize(&mut self, buffer: Vec<u8>);
    fn clear(&mut self);
}

#[derive(Debug)]
pub struct Profiler {
    benchmarks: HashMap<String,Vec<BatchSummary>>,
}

#[derive(Debug)]
struct Metric {
    values: Vec<f64>
}

#[derive(Debug)]
struct BatchSummary {
    batch_size: usize,
    uncompressed_size_byte: Summary,
    compressed_size_byte: Summary,
    batch_creation_sec: Summary,
    serialization_sec: Summary,
    deserialization_sec: Summary,
}

#[derive(Debug)]
struct Summary {
    min: f64,
    max: f64,
    mean: f64,
    stddev: f64,
    p50: f64,
    p90: f64,
    p95: f64,
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
            p95: self.percentile(95f64)
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
    pub fn new() -> Self {
        Self { benchmarks: HashMap::new() }
    }

    pub fn profile(&mut self, otel_impl: &mut impl ProfilableProtocol, batch_sizes: Vec<usize>, max_iter: usize) -> Result<(), Box<dyn Error>> {
        for batch_size in batch_sizes {
            println!("Profiling '{}' (batch-size={})", otel_impl.name(), batch_size);

            let mut uncompressed_size = Metric::new();
            let mut compressed_size = Metric::new();
            let mut batch_creation = Metric::new();
            let mut serialization = Metric::new();
            let mut deserialization = Metric::new();

            for _ in 0..max_iter {
                let max_batch_count = otel_impl.dataset_size() / batch_size;
                let mut start_at = 0;
                for _ in 0..max_batch_count {
                    // Batch creation
                    jemalloc_ctl::epoch().unwrap();
                    let start = Instant::now();
                    otel_impl.create_batch(start_at, batch_size);
                    let after_batch_creation = Instant::now();
                    let allocated = jemalloc_ctl::stats::allocated().unwrap();
                    let resident = jemalloc_ctl::stats::resident().unwrap();
                    println!("{} bytes allocated/{} bytes resident", allocated, resident);

                    // Serialization
                    let buffer = otel_impl.serialize()?;
                    uncompressed_size.record(buffer.len() as f64);
                    let after_serialization = Instant::now();

                    // Compression
                    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                    e.write_all(&buffer)?;
                    compressed_size.record(e.finish().unwrap().len() as f64);

                    // Deserialization
                    otel_impl.deserialize(buffer);
                    let after_deserialization = Instant::now();
                    otel_impl.clear();

                    start_at += batch_size;

                    batch_creation.record((after_batch_creation - start).as_secs_f64());
                    serialization.record((after_serialization - after_batch_creation).as_secs_f64());
                    deserialization.record((after_deserialization - after_serialization).as_secs_f64());
                }
            }

            self.benchmarks.entry(otel_impl.name())
                .and_modify(|vec| vec.push(BatchSummary {
                    batch_size,
                    uncompressed_size_byte: uncompressed_size.compute_summary(),
                    compressed_size_byte: compressed_size.compute_summary(),
                    batch_creation_sec: batch_creation.compute_summary(),
                    serialization_sec: serialization.compute_summary(),
                    deserialization_sec: deserialization.compute_summary(),
                }))
                .or_insert(vec![BatchSummary {
                    batch_size,
                    uncompressed_size_byte: uncompressed_size.compute_summary(),
                    compressed_size_byte: compressed_size.compute_summary(),
                    batch_creation_sec: batch_creation.compute_summary(),
                    serialization_sec: serialization.compute_summary(),
                    deserialization_sec: deserialization.compute_summary(),
                }]);
        }
        Ok(())
    }
}

