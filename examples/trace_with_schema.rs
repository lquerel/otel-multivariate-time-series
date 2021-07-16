use chrono::Utc;
use prost::Message;

use otel_multivariate_time_series::event::{BatchPolicy, EventBatchHandler, OpenTelemetryEvent};
use otel_multivariate_time_series::opentelemetry::proto::events::v1::{Int64Column, StringColumn, DoubleColumn, AuxiliaryEntity};
use otel_multivariate_time_series::opentelemetry::proto::events::v1::auxiliary_entity::LogicalType;

#[derive(Debug)]
pub enum SpanKind {
    SPAN_KIND_UNSPECIFIED = 0,
    SPAN_KIND_INTERNAL = 1,
    SPAN_KIND_SERVER = 2,
    SPAN_KIND_CLIENT = 3,
    SPAN_KIND_PRODUCER = 4,
    SPAN_KIND_CONSUMER = 5,
}

#[derive(Debug)]
pub struct Trace {
    pub trace_id: String,
    pub span_id: String,
    pub trace_state: String,
    pub parent_span_id: String,
    pub name: String,
    pub kind: SpanKind,
    pub start_time_nano: u64,
    pub end_time_nano: u64,
    pub attributes: TraceAttributes,
    // ToDo Should be Option<TraceAttributes>
    pub events: Vec<Event>,
    pub links: Vec<Link>,
    pub status: Status,
}

#[derive(Debug)]
pub enum StatusCode {
    STATUS_CODE_UNSET = 0,
    STATUS_CODE_OK = 1,
    STATUS_CODE_ERROR = 2,
}

#[derive(Debug)]
pub struct Status {
    pub message: String,
    pub code: StatusCode,
}

#[derive(Debug)]
pub struct TraceAttributes {
    pub http_user_agent: String,
    pub http_server_latency: i64,
    pub score: f64,
}

#[derive(Debug)]
pub struct Event {
    pub time_unix_nano: i64,
    pub name: String,
    pub attributes: EventAttributes,    // ToDo Should be Option<EventAttributes>
}

#[derive(Debug)]
pub struct EventAttributes {
    pub attribute_1: String,
    pub attribute_2: i64,
    pub attribute_3: f64,
}

#[derive(Debug)]
pub struct Link {
    pub trace_id: String,
    pub span_id: String,
    pub trace_state: String,
    pub attributes: LinkAttributes,     // ToDo Should be Option<LinkAttributes>
}

#[derive(Debug)]
pub struct LinkAttributes {
    pub attribute_1: String,
    pub attribute_2: f64,
}

#[derive(Debug)]
pub enum Attribute {
    I64 { key: String, value: i64},
    F64 { key: String, value: f64},
    String { key: String, value: String},
    Bool { key: String, value: bool},
}

impl OpenTelemetryEvent for Trace {
    fn urn() -> String {
        "urn:project_a:trace:service".into()
    }

    fn int64_columns(batch_policy: &BatchPolicy) -> Vec<Int64Column> where Self: Sized {
        vec![
            <Trace as OpenTelemetryEvent>::new_int64_column("kind", batch_policy),
            <Trace as OpenTelemetryEvent>::new_int64_column("attributes.http_server_latency", batch_policy),
            <Trace as OpenTelemetryEvent>::new_int64_column("status.code", batch_policy),
        ]
    }

    fn double_columns(batch_policy: &BatchPolicy) -> Vec<DoubleColumn> where Self: Sized {
        vec![
            <Trace as OpenTelemetryEvent>::new_double_column("attributes.score", batch_policy),
        ]
    }

    fn string_columns(batch_policy: &BatchPolicy) -> Vec<StringColumn> where Self: Sized {
        vec![
            <Trace as OpenTelemetryEvent>::new_string_column("trace_id", batch_policy),
            <Trace as OpenTelemetryEvent>::new_string_column("span_id", batch_policy),
            <Trace as OpenTelemetryEvent>::new_string_column("trace_state", batch_policy),
            <Trace as OpenTelemetryEvent>::new_string_column("parent_span_id", batch_policy),
            <Trace as OpenTelemetryEvent>::new_string_column("name", batch_policy),
            <Trace as OpenTelemetryEvent>::new_string_column("attributes.http_user_agent", batch_policy),
            <Trace as OpenTelemetryEvent>::new_string_column("status.message", batch_policy),
        ]
    }

    fn auxiliary_entities(batch_policy: &BatchPolicy) -> Vec<AuxiliaryEntity> where Self: Sized {
        vec![
            // Events
            AuxiliaryEntity {
                schema_url: "".to_string(),
                logical_type: LogicalType::TraceEvent as i32,
                size: 0,
                parent_column: "events".to_string(),
                parent_ranks: Vec::with_capacity(batch_policy.max_size as usize),
                i64_values: vec![
                    <Trace as OpenTelemetryEvent>::new_int64_column("time_unix_nano", batch_policy),
                    <Trace as OpenTelemetryEvent>::new_int64_column("attributes.attribute_2", batch_policy),
                ],
                f64_values: vec![
                    <Trace as OpenTelemetryEvent>::new_double_column("attributes.attribute_3", batch_policy),
                ],
                string_values: vec![
                    <Trace as OpenTelemetryEvent>::new_string_column("name", batch_policy),
                    <Trace as OpenTelemetryEvent>::new_string_column("attributes.attribute_1", batch_policy),
                ],
                bool_values: Vec::with_capacity(0),
                bytes_values: Vec::with_capacity(0),
                i64_summary_values: Vec::with_capacity(0),
                f64_summary_values: Vec::with_capacity(0),
            },
            // Links
            AuxiliaryEntity {
                schema_url: "".to_string(),
                logical_type: LogicalType::TraceLink as i32,
                size: 0,
                parent_column: "links".to_string(),
                parent_ranks: Vec::with_capacity(batch_policy.max_size as usize),
                i64_values: Vec::with_capacity(0),
                f64_values: vec![
                    <Trace as OpenTelemetryEvent>::new_double_column("attributes.attribute_2", batch_policy),
                ],
                string_values: vec![
                    <Trace as OpenTelemetryEvent>::new_string_column("trace_id", batch_policy),
                    <Trace as OpenTelemetryEvent>::new_string_column("span_id", batch_policy),
                    <Trace as OpenTelemetryEvent>::new_string_column("trace_state", batch_policy),
                    <Trace as OpenTelemetryEvent>::new_string_column("attributes.attribute_1", batch_policy),
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
            batch.i64_values[1].values.clear();
            batch.i64_values[2].values.clear();

            batch.f64_values[0].values.clear();

            batch.string_values[0].values.clear();
            batch.string_values[1].values.clear();
            batch.string_values[2].values.clear();
            batch.string_values[3].values.clear();
            batch.string_values[4].values.clear();
            batch.string_values[5].values.clear();
            batch.string_values[6].values.clear();
            batch.size = 0;

            let events = &mut batch.auxiliary_entities[0];
            events.parent_ranks.clear();
            events.i64_values[0].values.clear();
            events.i64_values[1].values.clear();
            events.f64_values[0].values.clear();
            events.string_values[0].values.clear();
            events.string_values[1].values.clear();
            events.size = 0;

            let links = &mut batch.auxiliary_entities[1];
            links.parent_ranks.clear();
            links.f64_values[0].values.clear();

            links.string_values[0].values.clear();
            links.string_values[1].values.clear();
            links.string_values[2].values.clear();
            links.string_values[3].values.clear();
            links.size = 0;
        }

        let batch = &mut handler.resource_events.instrumentation_library_events[0].batches[0];

        batch.start_time_unix_nano_column.push(self.start_time_nano);
        batch.end_time_unix_nano_column.push(self.end_time_nano);

        // Set top level columns
        batch.i64_values[0].values.push(self.kind as i64);
        batch.i64_values[1].values.push(self.attributes.http_server_latency);
        batch.i64_values[2].values.push(self.status.code as i64);

        batch.f64_values[0].values.push(self.attributes.score);

        batch.string_values[0].values.push(self.trace_id);
        batch.string_values[1].values.push(self.span_id);
        batch.string_values[2].values.push(self.trace_state);
        batch.string_values[3].values.push(self.parent_span_id);
        batch.string_values[4].values.push(self.name);
        batch.string_values[5].values.push(self.attributes.http_user_agent);
        batch.string_values[6].values.push(self.status.message);
        batch.size += 1;

        // Set auxiliary entities ====================================
        // Events
        let events = &mut batch.auxiliary_entities[0];
        events.size += self.events.len() as u32;
        for event in self.events {
            events.parent_ranks.push(batch.size-1);
            events.i64_values[0].values.push(event.time_unix_nano);
            events.i64_values[1].values.push(event.attributes.attribute_2);

            events.f64_values[0].values.push(event.attributes.attribute_3);

            events.string_values[0].values.push(event.name);
            events.string_values[1].values.push(event.attributes.attribute_1);
        }

        // Links
        let links = &mut batch.auxiliary_entities[1];
        links.size += self.links.len() as u32;
        for link in self.links {
            links.parent_ranks.push(batch.size-1);
            links.f64_values[0].values.push(link.attributes.attribute_2);

            links.string_values[0].values.push(link.trace_id);
            links.string_values[1].values.push(link.span_id);
            links.string_values[2].values.push(link.trace_state);
            links.string_values[3].values.push(link.attributes.attribute_1);
        }
    }
}