use std::error::Error;
use crate::multivariate_ts_gen::MultivariateDataPoint;
use crate::metrics_std::gen_standard_metrics;
use prost::Message;
use crate::metrics_columnar::gen_columnar_metrics;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::Write;
use std::time::Instant;
use crate::opentelemetry::proto::metrics::v1::ResourceMetrics;
use bytes::Bytes;

mod multivariate_ts_gen;
mod metrics_std;
mod metrics_columnar;

pub mod opentelemetry {
    pub mod proto {
        pub mod common {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.common.v1.rs"));
            }
        }

        pub mod resource {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.resource.v1.rs"));
            }
        }

        pub mod metrics {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.metrics.v1.rs"));
            }
            pub mod experimental {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.metrics.experimental.rs"));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for i in 1..10 {
        let time_series = MultivariateDataPoint::load_time_series("multivariate-time-series.json", i*1000)?;
        println!("Multivariate time-series experiment (batch of {} data points)", time_series.len());

        let before_gen_time = Instant::now();
        let resource_metrics = gen_standard_metrics(&time_series);
        let gen_time = Instant::now();
        let mut buf: Vec<u8> = Vec::new();
        let before_ser_time = Instant::now();
        resource_metrics.encode(&mut buf)?;
        let ser_time = Instant::now();
        let std_uncompressed_size = buf.len();
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(&buf)?;
        let compressed_bytes = e.finish().unwrap();
        let std_compressed_size = compressed_bytes.len();
        let before_deser_time = Instant::now();
        let resource_metrics = ResourceMetrics::decode(Bytes::from(buf)).unwrap();
        assert_eq!("tbd".to_string(), resource_metrics.schema_url);
        let deser_time = Instant::now();
        let std_gen_time = gen_time - before_gen_time;
        let std_ser_time = ser_time - before_ser_time;
        let std_deser_time = deser_time - before_deser_time;
        println!("Standard representation:");
        println!("\tuncompressed size: {} bytes", std_uncompressed_size);
        println!("\tcompressed size: {} bytes", std_compressed_size);
        println!("\tprotobuf creation time: {}s", (gen_time - before_gen_time).as_secs_f64());
        println!("\tprotobuf serialization time: {}s", (ser_time - before_ser_time).as_secs_f64());
        println!("\tprotobuf deserialization time: {}s", (deser_time - before_deser_time).as_secs_f64());
        println!();

        let before_gen_time = Instant::now();
        let resource_metrics = gen_columnar_metrics(&time_series);
        let gen_time = Instant::now();
        let mut buf: Vec<u8> = Vec::new();
        let before_ser_time = Instant::now();
        resource_metrics.encode(&mut buf)?;
        let ser_time = Instant::now();
        let uncompressed_size = buf.len();
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(&buf)?;
        let compressed_bytes = e.finish().unwrap();
        let compressed_size = compressed_bytes.len();
        let before_deser_time = Instant::now();
        let resource_metrics = ResourceMetrics::decode(Bytes::from(buf)).unwrap();
        assert_eq!("tbd".to_string(), resource_metrics.schema_url);
        let deser_time = Instant::now();
        println!("Columnar representation:");
        println!("\tuncompressed size: {} bytes\t\t\t\t({} times smaller)", uncompressed_size, std_uncompressed_size/uncompressed_size);
        println!("\tcompressed size: {} bytes\t\t\t\t({} times smaller)", compressed_size, std_compressed_size/compressed_size);
        println!("\tprotobuf creation time: {}s\t\t\t({} times faster)", (gen_time - before_gen_time).as_secs_f64(), std_gen_time.as_secs_f64()/(gen_time - before_gen_time).as_secs_f64());
        println!("\tprotobuf serialization time: {}s\t\t({} times faster)", (ser_time - before_ser_time).as_secs_f64(), std_ser_time.as_secs_f64()/(ser_time - before_ser_time).as_secs_f64());
        println!("\tprotobuf deserialization time: {}s\t\t({} times faster)", (deser_time - before_deser_time).as_secs_f64(), std_deser_time.as_secs_f64()/(deser_time - before_deser_time).as_secs_f64());

        println!();
    }

    Ok(())
}



