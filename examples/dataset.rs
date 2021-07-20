use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct Dataset<T> {
    pub values: Vec<T>,
}

impl <T> Dataset<T> where T: DeserializeOwned {
    pub fn new(data_file: &str) -> Self {
        let file = File::open(path_buf(data_file)).expect("data file not found");
        let reader = BufReader::new(file);

        let values: Vec<T> = serde_json::from_reader(reader).expect("data file not a readable JSON file");
        Self {
            values,
        }
    }
}

pub fn path_buf(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(format!("data/{}", file_name));
    path
}