use std::marker::PhantomData;
use crate::opentelemetry::proto::events::v1::{StringColumn, Int64Column, DoubleColumn, BytesColumn, Int64SummaryColumn, DoubleSummaryColumn, BoolColumn, ResourceEvents, InstrumentationLibraryEvents, BatchEvent, AuxiliaryEntity};
use crate::opentelemetry::proto::resource::v1::Resource;
use crate::opentelemetry::proto::common::v1::InstrumentationLibrary;
use serde_json::{Value, Number, Map};
use arrow::datatypes::Schema;
use crate::opentelemetry::proto::arrow_events::v1 as arrow_events;
use prost::{Message, EncodeError};
use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct BatchPolicy {
    pub max_size: u32,
    pub max_delay: chrono::Duration,
}

#[derive(Debug)]
pub struct EventBatchHandler<T: OpenTelemetryEvent> {
    // ToDo why do we need 3 schema urn (recursively)
    schema_url: String,
    // ToDo pub(crate) ?
    pub batch_policy: BatchPolicy,
    pub resource_events: ResourceEvents,
    phantom_data: PhantomData<T>,
}

#[derive(Debug)]
pub struct ArrowEventBatchHandler<T: OpenTelemetryArrowEvent> {
    // ToDo why do we need 3 schema urn (recursively)
    schema_url: String,
    // ToDo pub(crate) ?
    pub batch_policy: BatchPolicy,
    pub resource_events: arrow_events::ResourceEvents,
    pub arrow_schema: Schema,
    phantom_data: PhantomData<T>,
}

#[derive(Debug)]
pub struct EventCollector {
    default_batch_policy: BatchPolicy,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error (error: {0})")]
    IoError(#[from] std::io::Error),
}

pub trait OpenTelemetryEvent {
    fn urn() -> String;
    fn int64_columns(_batch_policy: &BatchPolicy) -> Vec<Int64Column> { Vec::with_capacity(0) }
    fn double_columns(_batch_policy: &BatchPolicy) -> Vec<DoubleColumn> { Vec::with_capacity(0) }
    fn string_columns(_batch_policy: &BatchPolicy) -> Vec<StringColumn> { Vec::with_capacity(0) }
    fn bool_columns(_batch_policy: &BatchPolicy) -> Vec<BoolColumn> { Vec::with_capacity(0) }
    fn bytes_columns(_batch_policy: &BatchPolicy) -> Vec<BytesColumn> { Vec::with_capacity(0) }
    fn int64_summary_columns(_batch_policy: &BatchPolicy) -> Vec<Int64SummaryColumn> { Vec::with_capacity(0) }
    fn double_summary_columns(_batch_policy: &BatchPolicy) -> Vec<DoubleSummaryColumn> { Vec::with_capacity(0) }
    fn auxiliary_entities(_batch_policy: &BatchPolicy) -> Vec<AuxiliaryEntity> where Self: Sized { Vec::with_capacity(0) }
    fn record_into(self, handler: &mut EventBatchHandler<Self>) where Self: Sized;

    fn new_int64_column(name: &str, batch_policy: &BatchPolicy) -> Int64Column {
        Int64Column {
            name: name.into(),
            logical_type: 0,
            description: "".to_string(),
            unit: "".to_string(),
            aggregation_temporality: 0,
            is_monotonic: false,
            values: Vec::with_capacity(batch_policy.max_size as usize),
            validity_bitmap: vec![],
        }
    }

    fn new_optional_int64_column(name: &str, batch_policy: &BatchPolicy) -> Int64Column {
        Int64Column {
            name: name.into(),
            logical_type: 0,
            description: "".to_string(),
            unit: "".to_string(),
            aggregation_temporality: 0,
            is_monotonic: false,
            values: Vec::with_capacity(batch_policy.max_size as usize),
            validity_bitmap: validity_bitmap(batch_policy.max_size as usize),
        }
    }

    fn new_double_column(name: &str, batch_policy: &BatchPolicy) -> DoubleColumn {
        DoubleColumn {
            name: name.into(),
            logical_type: 0,
            description: "".to_string(),
            unit: "".to_string(),
            aggregation_temporality: 0,
            is_monotonic: false,
            values: Vec::with_capacity(batch_policy.max_size as usize),
            validity_bitmap: vec![],
        }
    }

    fn new_string_column(name: &str, batch_policy: &BatchPolicy) -> StringColumn {
        StringColumn {
            name: name.into(),
            logical_type: 0,
            description: "".to_string(),
            values: Vec::with_capacity(batch_policy.max_size as usize),
            validity_bitmap: vec![],
        }
    }

    fn new_optional_string_column(name: &str, batch_policy: &BatchPolicy) -> StringColumn {
        StringColumn {
            name: name.into(),
            logical_type: 0,
            description: "".to_string(),
            values: Vec::with_capacity(batch_policy.max_size as usize),
            validity_bitmap: validity_bitmap(batch_policy.max_size as usize),
        }
    }
}

pub trait OpenTelemetryArrowEvent {
    fn urn() -> String;
    fn arrow_schema(_batch_policy: &BatchPolicy) -> Schema;
    fn record_into(self, handler: &mut ArrowEventBatchHandler<Self>) where Self: Sized;
}

impl BatchPolicy {
    pub fn new(max_size: u32, max_delay: chrono::Duration) -> Self {
        BatchPolicy { max_size, max_delay }
    }
}

impl<T> EventBatchHandler<T> where T: OpenTelemetryEvent {
    pub fn new(batch_policy: BatchPolicy) -> Self {
        EventBatchHandler {
            schema_url: T::urn(),
            batch_policy: batch_policy.clone(),
            phantom_data: PhantomData::default(),
            resource_events: ResourceEvents {
                resource: Some(Resource {
                    attributes: vec![],
                    dropped_attributes_count: 0,
                }),
                instrumentation_library_events: vec![
                    InstrumentationLibraryEvents {
                        instrumentation_library: Some(InstrumentationLibrary { name: "otel-rust".into(), version: "1.0".into() }),
                        batches: vec![
                            BatchEvent {
                                schema_url: T::urn(),
                                size: 0,
                                start_time_unix_nano_column: Vec::with_capacity(batch_policy.max_size as usize),
                                end_time_unix_nano_column: Vec::with_capacity(batch_policy.max_size as usize),
                                i64_values: T::int64_columns(&batch_policy),
                                f64_values: T::double_columns(&batch_policy),
                                string_values: T::string_columns(&batch_policy),
                                bool_values: T::bool_columns(&batch_policy),
                                bytes_values: T::bytes_columns(&batch_policy),
                                i64_summary_values: T::int64_summary_columns(&batch_policy),
                                f64_summary_values: T::double_summary_columns(&batch_policy),
                                auxiliary_entities: T::auxiliary_entities(&batch_policy),
                            }
                        ],
                        dropped_events_count: 0,
                    }
                ],
                schema_url: "".into(),
            },
        }
    }

    pub fn to_json_value(&self) -> Value {
        let mut values = vec![];

        for instrumentation_library_event in &self.resource_events.instrumentation_library_events {
            for batch_event in &instrumentation_library_event.batches {
                let first_event_rank = values.len();

                for i in 0..batch_event.size as usize {
                    let mut json_object = serde_json::Map::new();

                    json_object.insert("@schema_url".to_string(), Value::String(batch_event.schema_url.clone()));
                    json_object.insert("@start_time_unix_nano".to_string(), Value::Number(Number::from(batch_event.start_time_unix_nano_column[i])));
                    json_object.insert("@end_time_unix_nano".to_string(), Value::Number(Number::from(batch_event.end_time_unix_nano_column[i])));

                    Self::insert_i64_values(&mut json_object, &batch_event.i64_values, i);
                    Self::insert_f64_values(&mut json_object, &batch_event.f64_values, i);
                    Self::insert_string_values(&mut json_object, &batch_event.string_values, i);
                    Self::insert_bool_values(&mut json_object, &batch_event.bool_values, i);

                    values.push(Value::Object(json_object));
                }

                if !batch_event.auxiliary_entities.is_empty() {
                    for auxiliary_entity in &batch_event.auxiliary_entities {
                        let mut auxiliary_values = vec![];
                        let mut parent_rank: usize = 0;

                        for j in 0..auxiliary_entity.size as usize {
                            if parent_rank == auxiliary_entity.parent_ranks[j] as usize {
                                let mut json_object = serde_json::Map::new();

                                Self::insert_i64_values(&mut json_object, &auxiliary_entity.i64_values, j);
                                Self::insert_f64_values(&mut json_object, &auxiliary_entity.f64_values, j);
                                Self::insert_string_values(&mut json_object, &auxiliary_entity.string_values, j);
                                Self::insert_bool_values(&mut json_object, &auxiliary_entity.bool_values, j);

                                auxiliary_values.push(Value::Object(json_object));
                            } else {
                                if let Some(json_object) = values[first_event_rank + parent_rank].as_object_mut() {
                                    json_object.insert(auxiliary_entity.parent_column.clone(), Value::Array(auxiliary_values));
                                }
                                auxiliary_values = vec![];
                                parent_rank = auxiliary_entity.parent_ranks[j] as usize;
                                let mut json_object = serde_json::Map::new();

                                Self::insert_i64_values(&mut json_object, &auxiliary_entity.i64_values, j);
                                Self::insert_f64_values(&mut json_object, &auxiliary_entity.f64_values, j);
                                Self::insert_string_values(&mut json_object, &auxiliary_entity.string_values, j);
                                Self::insert_bool_values(&mut json_object, &auxiliary_entity.bool_values, j);

                                auxiliary_values.push(Value::Object(json_object));
                            }
                        }
                        if !auxiliary_values.is_empty() {
                            if let Some(json_object) = values[first_event_rank + parent_rank].as_object_mut() {
                                json_object.insert(auxiliary_entity.parent_column.clone(), Value::Array(auxiliary_values));
                            }
                        }
                    }
                }
            }
        }

        Value::Array(values)
    }

    fn insert_i64_values(json_object: &mut Map<String, Value>, i64_values: &[Int64Column], rank: usize) {
        for column in i64_values {
            if column.validity_bitmap.is_empty() || is_valid_value(&column.validity_bitmap, rank) {
                json_object.insert(column.name.clone(), Value::Number(Number::from(column.values[rank])));
            }
        }
    }

    fn insert_f64_values(json_object: &mut Map<String, Value>, f64_values: &[DoubleColumn], rank: usize) {
        for column in f64_values {
            if column.validity_bitmap.is_empty() || is_valid_value(&column.validity_bitmap, rank) {
                let number = Number::from_f64(column.values[rank]);
                if let Some(number) = number {
                    json_object.insert(column.name.clone(), Value::Number(number));
                }
            }
        }
    }

    fn insert_string_values(json_object: &mut Map<String, Value>, string_values: &[StringColumn], rank: usize) {
        for column in string_values {
            if column.validity_bitmap.is_empty() || is_valid_value(&column.validity_bitmap, rank) {
                json_object.insert(column.name.clone(), Value::String(column.values[rank].clone()));
            }
        }
    }

    fn insert_bool_values(json_object: &mut Map<String, Value>, bool_values: &[BoolColumn], rank: usize) {
        for column in bool_values {
            if column.validity_bitmap.is_empty() || is_valid_value(&column.validity_bitmap, rank) {
                json_object.insert(column.name.clone(), Value::Bool(column.values[rank]));
            }
        }
    }
}

impl EventCollector {
    pub fn new(batch_policy: BatchPolicy) -> Self {
        EventCollector {
            default_batch_policy: batch_policy,
        }
    }

    pub fn event_handler<T: OpenTelemetryEvent>(&self) -> EventBatchHandler<T> {
        EventBatchHandler::new(self.default_batch_policy.clone())
    }

    pub fn arrow_event_handler<T: OpenTelemetryArrowEvent>(&self) -> ArrowEventBatchHandler<T> {
        ArrowEventBatchHandler {
            schema_url: T::urn(),
            batch_policy: self.default_batch_policy.clone(),
            phantom_data: PhantomData::default(),
            resource_events: arrow_events::ResourceEvents {
                resource: Some(Resource {
                    attributes: vec![],
                    dropped_attributes_count: 0,
                }),
                instrumentation_library_events: vec![
                    arrow_events::InstrumentationLibraryEvents {
                        instrumentation_library: Some(InstrumentationLibrary { name: "otel-rust".into(), version: "1.0".into() }),
                        batches: vec![
                            arrow_events::BatchEvent {
                                schema_url: T::urn(),
                                size: 0,
                                start_time_unix_nano_column: Vec::with_capacity(self.default_batch_policy.max_size as usize),
                                end_time_unix_nano_column: Vec::with_capacity(self.default_batch_policy.max_size as usize),
                                arrow_buffer: vec![],
                            }
                        ],
                        dropped_events_count: 0,
                    }
                ],
                schema_url: "".into(),
            },
            arrow_schema: T::arrow_schema(&self.default_batch_policy),
        }
    }
}

impl<T: OpenTelemetryEvent> EventBatchHandler<T> {
    pub fn record(&mut self, event: T) -> Result<(), Error> {
        if self.resource_events.instrumentation_library_events[0].batches[0].size == self.batch_policy.max_size {
            self.reset_batch_event();
        }

        event.record_into(self);
        Ok(())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        self.resource_events.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn deserialize(&mut self, buf: Vec<u8>) {
        self.resource_events = ResourceEvents::decode(Bytes::from(buf)).unwrap();
    }

    pub fn reset_batch_event(&mut self) {
        let batch = &mut self.resource_events.instrumentation_library_events[0].batches[0];

        batch.start_time_unix_nano_column.clear();
        batch.end_time_unix_nano_column.clear();

        for column in batch.i64_values.iter_mut() {
            column.values.clear();
            reset_validity_bitmap(&mut column.validity_bitmap);
        }

        for column in batch.f64_values.iter_mut() {
            column.values.clear();
            reset_validity_bitmap(&mut column.validity_bitmap);
        }

        for column in batch.string_values.iter_mut() {
            column.values.clear();
            reset_validity_bitmap(&mut column.validity_bitmap);
        }

        batch.size = 0;

        for auxiliary_entity in batch.auxiliary_entities.iter_mut() {
            auxiliary_entity.parent_ranks.clear();

            for column in auxiliary_entity.i64_values.iter_mut() {
                column.values.clear();
                reset_validity_bitmap(&mut column.validity_bitmap);
            }

            for column in auxiliary_entity.f64_values.iter_mut() {
                column.values.clear();
                reset_validity_bitmap(&mut column.validity_bitmap);
            }

            for column in auxiliary_entity.string_values.iter_mut() {
                column.values.clear();
                reset_validity_bitmap(&mut column.validity_bitmap);
            }

            auxiliary_entity.size = 0;
        }

        let attributes = &mut batch.auxiliary_entities[0];
        attributes.parent_ranks.clear();
        attributes.string_values[0].values.clear();
        attributes.string_values[1].values.clear();
        attributes.size = 0;
    }
}

impl<T: OpenTelemetryArrowEvent> ArrowEventBatchHandler<T> {
    pub fn record(&mut self, event: T) -> Result<(), Error> {
        event.record_into(self);
        Ok(())
    }
}

/// Note: This invariant nth_bit/8 < bytes.len() is enforced by design (code generated by the macro).
#[inline(always)]
pub fn set_nth_bit(validity_bitmap: &mut Vec<u8>, nth_bit: usize) {
    validity_bitmap[nth_bit / 8] |= 1 << (nth_bit % 8);
}

#[inline(always)]
pub fn is_valid_value(validity_bitmap: &Vec<u8>, nth_bit: usize) -> bool {
    validity_bitmap[nth_bit / 8] & (1 << (nth_bit % 8)) > 0
}

#[inline(always)]
pub fn validity_bitmap(size: usize) -> Vec<u8> {
    vec![0; size/8 + if size%8 > 0 { 1} else {0}]
}

#[inline(always)]
pub fn reset_validity_bitmap(validity_bitmap: &mut Vec<u8>) {
    for byte in validity_bitmap {
        *byte = 0;
    }
}

#[cfg(test)]
mod test {
    use crate::event::{set_nth_bit, validity_bitmap, reset_validity_bitmap};

    #[test]
    fn test() {
        let size = (100 + (8 - 1)) / 8;
        let mut validity_bitmap: Vec<u8> = validity_bitmap(size);

        set_nth_bit(&mut validity_bitmap, 0);
        set_nth_bit(&mut validity_bitmap, 2);
        set_nth_bit(&mut validity_bitmap, 10);

        assert_eq!(&format!("{:08b}", validity_bitmap[0]), "00000101");
        assert_eq!(&format!("{:08b}", validity_bitmap[1]), "00000100");
        assert_eq!(&format!("{:08b}", validity_bitmap[2]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[3]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[4]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[5]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[6]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[7]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[8]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[9]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[10]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[11]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[12]), "00000000");

        reset_validity_bitmap(&mut validity_bitmap);
        assert_eq!(&format!("{:08b}", validity_bitmap[0]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[1]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[2]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[3]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[4]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[5]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[6]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[7]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[8]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[9]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[10]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[11]), "00000000");
        assert_eq!(&format!("{:08b}", validity_bitmap[12]), "00000000");
    }
}
