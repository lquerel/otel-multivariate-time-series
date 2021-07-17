use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde::Deserialize;
use otel_multivariate_time_series::event::{BatchPolicy, ArrowEventBatchHandler, reset_validity_bitmap, set_nth_bit, OpenTelemetryArrowEvent, OpenTelemetryEvent, EventBatchHandler};
use prost::Message;
use arrow::datatypes::{Schema, Field, DataType};
use otel_multivariate_time_series::opentelemetry::proto::events::v1::{Int64Column, StringColumn, AuxiliaryEntity};
use otel_multivariate_time_series::opentelemetry::proto::events::v1::auxiliary_entity::LogicalType;

#[derive(Deserialize, Debug, Clone)]
pub struct JsonTrace {
    pub evt: Evt,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Evt {
    pub trace_id: String,
    pub span_id: String,
    pub trace_state: Option<String>,
    pub parent_span_id: Option<String>,
    pub name: String,
    pub kind: Option<i64>,
    pub start_time_utc: DateTime<Utc>,
    pub end_time_utc: DateTime<Utc>,
    pub status: Status,
    pub attributes: Option<HashMap<String,Option<String>>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    pub message: Option<String>,
    pub code: Option<i64>

}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::fs::File;
    use crate::JsonTrace;

    pub fn data_file(file_name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("data/{}", file_name));
        path
    }

    #[test]
    fn test_json_trace() -> Result<(), Box<dyn Error>>{
        let file = File::open(data_file("trace_samples.json"))?;
        let reader = BufReader::new(file);

        let json_traces: Vec<JsonTrace> = serde_json::from_reader(reader)?;

        assert_eq!(json_traces.len(),15000);
        Ok(())
    }
}

impl OpenTelemetryEvent for JsonTrace {
    fn urn() -> String {
        "urn:project_a:trace:service".into()
    }

    fn int64_columns(batch_policy: &BatchPolicy) -> Vec<Int64Column> where Self: Sized {
        vec![
            <JsonTrace as OpenTelemetryEvent>::new_optional_int64_column("kind", batch_policy),
            <JsonTrace as OpenTelemetryEvent>::new_optional_int64_column("status.code", batch_policy),
        ]
    }

    fn string_columns(batch_policy: &BatchPolicy) -> Vec<StringColumn> where Self: Sized {
        vec![
            <JsonTrace as OpenTelemetryEvent>::new_string_column("trace_id", batch_policy),
            <JsonTrace as OpenTelemetryEvent>::new_string_column("span_id", batch_policy),
            <JsonTrace as OpenTelemetryEvent>::new_optional_string_column("trace_state", batch_policy),
            <JsonTrace as OpenTelemetryEvent>::new_optional_string_column("parent_span_id", batch_policy),
            <JsonTrace as OpenTelemetryEvent>::new_string_column("name", batch_policy),
            <JsonTrace as OpenTelemetryEvent>::new_optional_string_column("status.message", batch_policy),
        ]
    }

    fn auxiliary_entities(batch_policy: &BatchPolicy) -> Vec<AuxiliaryEntity> where Self: Sized {
        vec![
            AuxiliaryEntity {
                schema_url: "".to_string(),
                logical_type: LogicalType::Attribute as i32,
                size: 0,
                parent_column: "attributes".to_string(),
                parent_ranks: Vec::with_capacity(batch_policy.max_size as usize),
                i64_values: Vec::with_capacity(0),
                f64_values: Vec::with_capacity(0),
                string_values: vec![
                    <JsonTrace as OpenTelemetryEvent>::new_string_column("name", batch_policy),
                    <JsonTrace as OpenTelemetryEvent>::new_string_column("value", batch_policy),
                ],
                bool_values: Vec::with_capacity(0),
                bytes_values: Vec::with_capacity(0),
                i64_summary_values: Vec::with_capacity(0),
                f64_summary_values: Vec::with_capacity(0),
            },
        ]
    }

    fn record_into(self, handler: &mut EventBatchHandler<Self>) where Self: Sized {
        if handler.resource_events.instrumentation_library_events[0].batches[0].size == handler.batch_policy.max_size {
            let mut buf: Vec<u8> = Vec::with_capacity(200);

            handler.resource_events.encode(&mut buf).unwrap();
            println!("{}", buf.len());

            let batch = &mut handler.resource_events.instrumentation_library_events[0].batches[0];

            batch.start_time_unix_nano_column.clear();
            batch.end_time_unix_nano_column.clear();

            batch.i64_values[0].values.clear();
            reset_validity_bitmap(&mut batch.i64_values[0].validity_bitmap);
            batch.i64_values[1].values.clear();
            reset_validity_bitmap(&mut batch.i64_values[1].validity_bitmap);

            batch.string_values[0].values.clear();
            batch.string_values[1].values.clear();
            batch.string_values[2].values.clear();
            reset_validity_bitmap(&mut batch.string_values[2].validity_bitmap);
            batch.string_values[3].values.clear();
            reset_validity_bitmap(&mut batch.string_values[3].validity_bitmap);
            batch.string_values[4].values.clear();
            batch.string_values[5].values.clear();
            reset_validity_bitmap(&mut batch.string_values[5].validity_bitmap);
            batch.size = 0;

            let attributes = &mut batch.auxiliary_entities[0];
            attributes.parent_ranks.clear();
            attributes.string_values[0].values.clear();
            attributes.string_values[1].values.clear();
            attributes.size = 0;
        }

        let batch = &mut handler.resource_events.instrumentation_library_events[0].batches[0];

        batch.start_time_unix_nano_column.push(self.evt.start_time_utc.timestamp_nanos() as u64);
        batch.end_time_unix_nano_column.push(self.evt.end_time_utc.timestamp_nanos() as u64);

        // Set top level columns
        match self.evt.kind {
            None => {
                batch.i64_values[0].values.push(0);
            }
            Some(kind) => {
                batch.i64_values[0].values.push(kind);
                let nth_bit = batch.i64_values[0].values.len() -1;
                set_nth_bit(&mut batch.i64_values[0].validity_bitmap, nth_bit);
            }
        }
        match self.evt.status.code {
            None => {
                batch.i64_values[1].values.push(0);
            }
            Some(code) => {
                batch.i64_values[1].values.push(code);
                let nth_bit = batch.i64_values[1].values.len() -1;
                set_nth_bit(&mut batch.i64_values[1].validity_bitmap, nth_bit);
            }
        }


        batch.string_values[0].values.push(self.evt.trace_id);
        batch.string_values[1].values.push(self.evt.span_id);
        match self.evt.trace_state {
            None => {
                batch.string_values[2].values.push("".into());
            }
            Some(trace_state) => {
                batch.string_values[2].values.push(trace_state);
                let nth_bit = batch.string_values[2].values.len() -1;
                set_nth_bit(&mut batch.string_values[2].validity_bitmap, nth_bit);
            }
        }
        match self.evt.parent_span_id {
            None => {
                batch.string_values[3].values.push("".into());
            }
            Some(parent_span_id) => {
                batch.string_values[3].values.push(parent_span_id);
                let nth_bit = batch.string_values[3].values.len() -1;
                set_nth_bit(&mut batch.string_values[3].validity_bitmap, nth_bit);
            }
        }
        batch.string_values[4].values.push(self.evt.name);
        match self.evt.status.message {
            None => {
                batch.string_values[5].values.push("".into());
            }
            Some(message) => {
                batch.string_values[5].values.push(message);
                let nth_bit = batch.string_values[5].values.len() -1;
                set_nth_bit(&mut batch.string_values[5].validity_bitmap, nth_bit);
            }
        }
        batch.size += 1;

        // Set auxiliary entities ====================================
        let aux_attributes = &mut batch.auxiliary_entities[0];
        if let Some(attributes) = self.evt.attributes {
            for (name, value) in attributes {
                if let Some(value) = value {
                    aux_attributes.parent_ranks.push(batch.size-1);
                    aux_attributes.string_values[0].values.push(name);
                    aux_attributes.string_values[1].values.push(value);
                    aux_attributes.size += 1;
                }
            }
        }
    }
}

impl OpenTelemetryArrowEvent for JsonTrace {
    fn urn() -> String {
        "urn:project_a:trace:service".into()
    }

    fn record_into(self, handler: &mut ArrowEventBatchHandler<Self>) where Self: Sized {
    }

    fn arrow_schema(_batch_policy: &BatchPolicy) -> Schema {
        Schema::new(vec![
            Field::new("kind", DataType::Int64, true),
            Field::new(
                "status",
                DataType::Struct(vec![
                    Field::new("message", DataType::Utf8, true),
                    Field::new("code", DataType::Int64, true),
                ]),
                true,
            ),
            Field::new("trace_id", DataType::Utf8, false),
            Field::new("span_id", DataType::Utf8, false),
            Field::new("trace_state", DataType::Utf8, true),
            Field::new("parent_span_id", DataType::Utf8, true),
            Field::new("name", DataType::Utf8, false),
            Field::new("attributes", DataType::List(
                Box::new(Field::new(
                    "attr",
                    DataType::Struct(vec![
                        Field::new("name", DataType::Utf8, false),
                        Field::new("value", DataType::Utf8, false),
                    ]),
                    true,
                ))
            ), true),
        ])
    }
}

/*
        let struct_type = DataType::Struct(vec![
            Field::new("name", DataType::Utf8, false),
            Field::new("value", DataType::Utf8, false),
        ]);
        let value_data = ArrayData::builder(struct_type.clone())
            .len(8)
            .add_buffer(Buffer::from())
            .build();
        let value_offsets = Buffer::from(&[0, 3, 6, 8].to_byte_slice());

        let list_data = ArrayData::builder(DataType::List(Box::new(Field::new(
            "attr",
            DataType::Struct(vec![
                Field::new("name", DataType::Utf8, false),
                Field::new("value", DataType::Utf8, false),
            ]),
            true,
        )))
            .len(10)
            .add_buffer(value_offsets)
            .add_child_data(value_data)
            .build();

        let list_array = ListArray::from(list_data);
 */