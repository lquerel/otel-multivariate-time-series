use std::time::{Instant, Duration};
use prost::EncodeError;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use std::error::Error;

pub trait ProfilableProtocol {
    fn dataset_size(&self) -> usize;
    fn create_batch(&mut self, start_at: usize, size: usize);
    fn serialize(&self) -> Result<Vec<u8>,EncodeError>;
    fn deserialize(&mut self, buffer: Vec<u8>);
    fn clear(&mut self);
}

#[derive(Debug)]
pub struct Profiler {
    measurements: Vec<Metrics>,
}

#[derive(Debug)]
struct Metrics {
    batch_size: usize,
    uncompressed_size: usize,
    compressed_size: usize,
    batch_creation: Duration,
    serialization: Duration,
    deserialization: Duration,
}

impl Profiler {
    pub fn new() -> Self {
        Self { measurements: vec![] }
    }

    pub fn profile(&mut self, otel_impl: &mut impl ProfilableProtocol, batch_sizes: Vec<usize>, max_iter: usize) -> Result<(), Box<dyn Error>> {
        for batch_size in batch_sizes {
            println!("Profiling batch size of {}", batch_size);
            for _ in 0..max_iter {
                let max_batch_count = otel_impl.dataset_size() / batch_size;
                let mut start_at = 0;
                for _ in 0..max_batch_count {
                    // Batch creation
                    let start = Instant::now();
                    otel_impl.create_batch(start_at, batch_size);
                    let after_batch_creation = Instant::now();

                    // Serialization
                    let buffer = otel_impl.serialize()?;
                    let uncompressed_size = buffer.len();
                    let after_serialization = Instant::now();

                    // Compression
                    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                    e.write_all(&buffer)?;
                    let compressed_size = e.finish().unwrap().len();

                    // Deserialization
                    otel_impl.deserialize(buffer);
                    let after_deserialization = Instant::now();
                    otel_impl.clear();

                    start_at += batch_size;

                    self.measurements.push(Metrics {
                        batch_size,
                        uncompressed_size,
                        compressed_size,
                        batch_creation: after_batch_creation - start,
                        serialization: after_serialization - after_batch_creation,
                        deserialization: after_deserialization - after_serialization,
                    });
                }
            }
        }
        Ok(())
    }
}

