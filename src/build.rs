use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    prost_build::compile_protos(&["proto/opentelemetry/proto/metrics/v1/metrics.proto"], &["proto/"])?;
    Ok(())
}