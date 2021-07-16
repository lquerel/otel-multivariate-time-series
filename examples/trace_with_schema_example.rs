use chrono::Utc;

use trace_with_schema::{Trace, StatusCode, SpanKind, Event, Link, TraceAttributes, EventAttributes, LinkAttributes, Status};
use otel_multivariate_time_series::event::{BatchPolicy, Error, EventCollector, EventBatchHandler, OpenTelemetryEvent};
use otel_multivariate_time_series::opentelemetry::proto::events::v1::{Int64Column, StringColumn};

mod trace_with_schema;

/*

What do I want to demonstrate? ==> Benefits = Efficiency + Versality + Extensibility
1 - Efficiency of the approach in term of CPU, memory (both side) and bandwidth.
  - Benchmarks between the existing standard OTEL v1 and this new implementation for metrics, and traces (optionally logs).
2 - Versality of the approach, i.e. the ability to represent any king of metrics (univariate and multivariate), logs and traces with a
  single internal and optimized representation.
3 - Extensibility of the approach
  - Dictionary encoding

Document the protobuf file
Update the README.md to summarize the approach, explain the benchmark approach and expose the results. This page must
highlight the benefits, and demonstrate the most important achievements.

Next evolution
- A RUST client using an advanced procedural macro to easily and efficiently report metrics, events and traces
- A RUST server
- Refactor the existing SDK to optionally use the new event representation for the existing methods to report metrics, logs, and traces
- More advanced encoding
- Use gRPC zero-copy options as much as possible
- A processor to filter/enrich/aggregate on the fly OTEL events
- A stream version as opposed to the existing query/response approach

 */
// ToDo Minimize allocations and branches
// ToDo Serialize and test perf
// ToDo Add Option support with validity_bitmap
// Support attributes
// Support validity_bitmap

fn main() -> Result<(), Error>{
    let event_collector = EventCollector::new(BatchPolicy::new(10, chrono::Duration::seconds(10)));
    let mut trace_handler: EventBatchHandler<Trace> = event_collector.event_handler();

    for trace_id in 0..2 {
        for span_id in 0..1 {
            trace_handler.record(Trace {
                trace_id: format!("trace_{}", trace_id),
                span_id: format!("span_{}", span_id),
                trace_state: "RequestProcessed".to_string(),
                parent_span_id: if trace_id > 0 {format!("span_{}", trace_id-1)} else { "".into() },
                name: format!("operation_{}", span_id),
                kind: SpanKind::SPAN_KIND_SERVER,
                start_time_nano: Utc::now().timestamp_nanos() as u64,
                end_time_nano: Utc::now().timestamp_nanos() as u64 +5,
                attributes: TraceAttributes {
                    http_user_agent: "chrome:01234".to_string(),
                    http_server_latency: 30,
                    score: 1.0
                },
                events: vec![
                    Event {
                        time_unix_nano: Utc::now().timestamp_nanos(),
                        name: "event1".to_string(),
                        attributes: EventAttributes {
                            attribute_1: "val1".to_string(),
                            attribute_2: 10,
                            attribute_3: 20.0
                        }
                    },
                    Event {
                        time_unix_nano: Utc::now().timestamp_nanos(),
                        name: "event2".to_string(),
                        attributes: EventAttributes {
                            attribute_1: "val2".to_string(),
                            attribute_2: 20,
                            attribute_3: 30.0
                        }
                    }
                ],
                links: vec![
                    Link {
                        trace_id: format!("trace_{}", trace_id-1),
                        span_id: format!("span_{}", span_id),
                        trace_state: "Processed".to_string(),
                        attributes: LinkAttributes { attribute_1: "val".to_string(), attribute_2: 10.0 }
                    }
                ],
                status: Status { message: "OK".to_string(), code: StatusCode::STATUS_CODE_OK }
            })?;
        }
    }

    println!("{:#?}", trace_handler);

    Ok(())
}

