use crate::opentelemetry::proto::trace::v1::{ResourceSpans, InstrumentationLibrarySpans, Span};
use crate::opentelemetry::proto::resource::v1::Resource;
use crate::opentelemetry::proto::common::v1::InstrumentationLibrary;
use prost::{Message, EncodeError};
use bytes::Bytes;

pub struct NativeTraceHandler {
    pub resource_spans: ResourceSpans,
}

impl NativeTraceHandler {
    pub fn new() -> Self {
        Self {
            resource_spans: ResourceSpans {
                resource: Some(Resource {
                    attributes: vec![],
                    dropped_attributes_count: 0,
                }),
                instrumentation_library_spans: vec![
                    InstrumentationLibrarySpans {
                        instrumentation_library: Some(InstrumentationLibrary { name: "otel-rust".into(), version: "1.0".into() }),
                        spans: vec![],
                        schema_url: "".into()
                    }
                ],
                schema_url: "".into()
            },
        }
    }

    pub fn record(&mut self, span: Span) {
        self.resource_spans.instrumentation_library_spans[0].spans.push(span);
    }

    pub fn clear(&mut self) {
        self.resource_spans.instrumentation_library_spans[0].spans.clear();
    }

    pub fn serialize(&self) -> Result<Vec<u8>,EncodeError> {
        let mut buf: Vec<u8> = Vec::new();
        self.resource_spans.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn deserialize(&mut self, buf: Vec<u8>) {
        self.resource_spans = ResourceSpans::decode(Bytes::from(buf)).unwrap();
    }
}

