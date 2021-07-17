use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::JsonTrace;

pub struct Dataset {
    pub traces: Vec<JsonTrace>,
}

impl Dataset {
    pub fn new(trace_file: &str) -> Self {
        let file = File::open(data_file(trace_file)).expect("trace file not found");
        let reader = BufReader::new(file);

        let traces: Vec<JsonTrace> = serde_json::from_reader(reader).expect("trace file not a readable JSON file");
        Self {
            traces,
        }
    }
}

pub fn data_file(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("data/{}", file_name));
    path
}