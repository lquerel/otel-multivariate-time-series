use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct MultivariateDataPoint {
    pub ts: DateTime<Utc>,
    pub source_id: String,
    pub evt: Event,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub fields: Fields,
    pub tags: Tags,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    pub method: String,
    #[serde(rename="dnsLookupMs_label_class")]
    pub dns_lookup_ms_label_class: String,
    pub source: String,
    pub url: String,
    #[serde(rename="tlsHandshakeMs_label_class")]
    pub tls_handshake_ms_label_class: String,
    pub remote_address: String,
    #[serde(rename="contentTransferMs_label_class")]
    pub content_transfer_ms_label_class: String,
    #[serde(rename="serverProcessingMs_label_class")]
    pub server_processing_ms_label_class: String,
    #[serde(rename="tcpConnectionMs_label_class")]
    pub tcp_connection_ms_label_class: String,
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    #[serde(rename="dnsLookupMs")]
    pub dns_lookup_ms: i64,
    #[serde(rename="serverProcessingMs")]
    pub server_processing_ms: i64,
    #[serde(rename="healthStatus")]
    pub health_status: i64,
    #[serde(rename="tcpConnectionMs")]
    pub tcp_connection_ms: i64,
    #[serde(rename="tlsHandshakeMs")]
    pub tls_handshake_ms: i64,
    #[serde(rename="failureCount")]
    pub failure_count: i64,
    pub size: i64,
    #[serde(rename="contentTransferMs")]
    pub content_transfer_ms: i64,
}

impl MultivariateDataPoint {
    pub fn load_time_series(file: &str, point_count: usize) -> Result<Vec<MultivariateDataPoint>,Box<dyn Error>> {
        let mut time_series = Vec::new();

        let file = File::open(file)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let data_point: MultivariateDataPoint = serde_json::from_str(&line?).unwrap();
            time_series.push(data_point);
            if time_series.len() == point_count {
                break;
            }
        }

        Ok(time_series)
    }
}