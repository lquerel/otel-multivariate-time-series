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
use plotters::prelude::*;

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
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut std_uncompressed_size_vec = vec![];
    let mut columnar_uncompressed_size_vec = vec![];
    let mut std_compressed_size_vec = vec![];
    let mut columnar_compressed_size_vec = vec![];
    let mut std_creation_time_vec = vec![];
    let mut columnar_creation_time_vec = vec![];
    let mut std_ser_time_vec = vec![];
    let mut columnar_ser_time_vec = vec![];
    let mut std_deser_time_vec = vec![];
    let mut columnar_deser_time_vec = vec![];

    for i in 1..10 {
        let time_series = MultivariateDataPoint::load_time_series("multivariate-time-series.json", i * 1000)?;
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
        std_uncompressed_size_vec.push(((i * 1000) as i64, std_uncompressed_size as i64));
        std_compressed_size_vec.push(((i * 1000) as i64, std_compressed_size as i64));
        std_creation_time_vec.push(((i * 1000) as i64, (gen_time - before_gen_time).as_nanos()));
        std_ser_time_vec.push(((i * 1000) as i64, (ser_time - before_ser_time).as_micros()));
        std_deser_time_vec.push(((i * 1000) as i64, (deser_time - before_deser_time).as_micros()));
        println!("Standard representation:");
        println!("\tuncompressed size: {} bytes", std_uncompressed_size);
        println!("\tcompressed size: {} bytes", std_compressed_size);
        println!("\tprotobuf creation time: {}μs", (gen_time - before_gen_time).as_nanos());
        println!("\tprotobuf serialization time: {}μs", (ser_time - before_ser_time).as_micros());
        println!("\tprotobuf deserialization time: {}μs", (deser_time - before_deser_time).as_micros());
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
        columnar_uncompressed_size_vec.push(((i * 1000) as i64, uncompressed_size as i64));
        columnar_compressed_size_vec.push(((i * 1000) as i64, compressed_size as i64));
        columnar_creation_time_vec.push(((i * 1000) as i64,(gen_time - before_gen_time).as_micros()));
        columnar_ser_time_vec.push(((i * 1000) as i64, (ser_time - before_ser_time).as_micros()));
        columnar_deser_time_vec.push(((i * 1000) as i64, (deser_time - before_deser_time).as_micros()));
        println!("Columnar representation:");
        println!("\tuncompressed size: {} bytes\t\t\t\t({} times smaller)", uncompressed_size, std_uncompressed_size / uncompressed_size);
        println!("\tcompressed size: {} bytes\t\t\t\t({} times smaller)", compressed_size, std_compressed_size / compressed_size);
        println!("\tprotobuf creation time: {}μs\t\t\t({} times faster)", (gen_time - before_gen_time).as_micros(), std_gen_time.as_micros() / (gen_time - before_gen_time).as_micros());
        println!("\tprotobuf serialization time: {}μs\t\t({} times faster)", (ser_time - before_ser_time).as_micros(), std_ser_time.as_micros() / (ser_time - before_ser_time).as_micros());
        println!("\tprotobuf deserialization time: {}μs\t\t({} times faster)", (deser_time - before_deser_time).as_micros(), std_deser_time.as_micros() / (deser_time - before_deser_time).as_micros());

        println!();
    }

    build_charts(&std_uncompressed_size_vec, &columnar_uncompressed_size_vec,
                 &std_compressed_size_vec, &columnar_compressed_size_vec,
                 &std_creation_time_vec, &columnar_creation_time_vec,
                 &std_ser_time_vec, &columnar_ser_time_vec,
                 &std_deser_time_vec, &columnar_deser_time_vec,
    );
    Ok(())
}

pub fn build_charts(std_uncompressed_size_vec: &[(i64, i64)],
                    columnar_uncompressed_size_vec: &[(i64, i64)],
                    std_compressed_size_vec: &[(i64, i64)],
                    columnar_compressed_size_vec: &[(i64, i64)],
                    std_create_time_vec: &[(i64, u128)],
                    columnar_create_time_vec: &[(i64, u128)],
                    std_ser_time_vec: &[(i64, u128)],
                    columnar_ser_time_vec: &[(i64, u128)],
                    std_deser_time_vec: &[(i64, u128)],
                    columnar_deser_time_vec: &[(i64, u128)],
) {
    let root_area = BitMapBackend::new("./images/charts.png", (1200, 768))
        .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let areas = root_area.split_evenly((3, 2));

    let mut ctx = ChartBuilder::on(&areas[0])
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Proto serialization (uncompressed in bytes)", ("sans-serif", 14))
        .build_cartesian_2d(0i64..10000i64, 0..std::cmp::max(
            std_uncompressed_size_vec.iter().map(|(_, y)| *y).max().unwrap(),
            columnar_uncompressed_size_vec.iter().map(|(_, y)| *y).max().unwrap(),
        ))
        .unwrap();

    ctx.configure_mesh()
        .x_desc("batch size")
        .y_desc("uncompressed size (bytes)")
        .draw().unwrap();

    ctx.draw_series(LineSeries::new(
        std_uncompressed_size_vec.iter().map(|(x, y)| (*x, *y)),
        &RED,
    )).unwrap().label("standard").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(
        columnar_uncompressed_size_vec.iter().map(|(x, y)| (*x, *y)),
        &BLUE,
    )).unwrap().label("columnar").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    // Compressed size chart
    let mut ctx = ChartBuilder::on(&areas[1])
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Proto serialization (compressed in bytes)", ("sans-serif", 14))
        .build_cartesian_2d(0i64..10000i64, 0..std::cmp::max(
            std_compressed_size_vec.iter().map(|(_, y)| *y).max().unwrap(),
            columnar_compressed_size_vec.iter().map(|(_, y)| *y).max().unwrap(),
        ))
        .unwrap();

    ctx.configure_mesh()
        .x_desc("batch size")
        .y_desc("compressed size (bytes)")
        .draw().unwrap();

    ctx.draw_series(LineSeries::new(
        std_compressed_size_vec.iter().map(|(x, y)| (*x, *y)),
        &RED,
    )).unwrap().label("standard").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(
        columnar_compressed_size_vec.iter().map(|(x, y)| (*x, *y)),
        &BLUE,
    )).unwrap().label("columnar").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    // Creation time chart
    let mut ctx = ChartBuilder::on(&areas[2])
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Proto msg create time", ("sans-serif", 14))
        .build_cartesian_2d(0i64..10000i64, 0..std::cmp::max(
            std_create_time_vec.iter().map(|(_, y)| *y).max().unwrap(),
            columnar_create_time_vec.iter().map(|(_, y)| *y).max().unwrap(),
        ))
        .unwrap();

    ctx.configure_mesh()
        .x_desc("batch size")
        .y_desc("create time (μs)")
        .draw().unwrap();

    ctx.draw_series(LineSeries::new(
        std_create_time_vec.iter().map(|(x, y)| (*x, *y)),
        &RED,
    )).unwrap().label("standard").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(
        columnar_create_time_vec.iter().map(|(x, y)| (*x, *y)),
        &BLUE,
    )).unwrap().label("columnar").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    // Creation time chart
    let mut ctx = ChartBuilder::on(&areas[4])
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Proto serialization time", ("sans-serif", 14))
        .build_cartesian_2d(0i64..10000i64, 0..std::cmp::max(
            std_ser_time_vec.iter().map(|(_, y)| *y).max().unwrap(),
            columnar_ser_time_vec.iter().map(|(_, y)| *y).max().unwrap(),
        ))
        .unwrap();

    ctx.configure_mesh()
        .x_desc("batch size")
        .y_desc("serialization time (μs)")
        .draw().unwrap();

    ctx.draw_series(LineSeries::new(
        std_ser_time_vec.iter().map(|(x, y)| (*x, *y)),
        &RED,
    )).unwrap().label("standard").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(
        columnar_ser_time_vec.iter().map(|(x, y)| (*x, *y)),
        &BLUE,
    )).unwrap().label("columnar").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();

    // Deser time chart
    let mut ctx = ChartBuilder::on(&areas[5])
        .set_label_area_size(LabelAreaPosition::Left, 70)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Proto deserialization time", ("sans-serif", 14))
        .build_cartesian_2d(0i64..10000i64, 0..std::cmp::max(
            std_deser_time_vec.iter().map(|(_, y)| *y).max().unwrap(),
            columnar_deser_time_vec.iter().map(|(_, y)| *y).max().unwrap(),
        ))
        .unwrap();

    ctx.configure_mesh()
        .x_desc("batch size")
        .y_desc("deserialization time (μs)")
        .draw().unwrap();

    ctx.draw_series(LineSeries::new(
        std_deser_time_vec.iter().map(|(x, y)| (*x, *y)),
        &RED,
    )).unwrap().label("standard").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(
        columnar_deser_time_vec.iter().map(|(x, y)| (*x, *y)),
        &BLUE,
    )).unwrap().label("columnar").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}



