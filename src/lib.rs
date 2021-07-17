pub mod multivariate_ts_gen;
pub mod metrics_std;
pub mod metrics_columnar;
pub mod event;
pub mod serializer;
pub mod error;
pub mod native_trace;

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

        pub mod trace {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.trace.v1.rs"));
            }
        }

        pub mod events {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.events.v1.rs"));
            }
        }

        pub mod arrow_events {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/opentelemetry.proto.arrow_events.v1.rs"));
            }
        }
    }
}
