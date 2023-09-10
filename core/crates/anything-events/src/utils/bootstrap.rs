use opentelemetry::{
    global,
    runtime::Tokio,
    sdk::{propagation::TraceContextPropagator, trace, Resource},
    KeyValue,
};
use std::fs;

use opentelemetry_otlp::WithExportConfig;
use tracing::Subscriber;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{fmt, prelude::*};

use anything_core::error::AnythingResult;
use tracing::info;

use crate::config::AnythingEventsConfig;

pub async fn bootstrap(config: &AnythingEventsConfig) -> AnythingResult<()> {
    info!("Bootstrapping Eventurous");
    // Bootstrap database directory
    let root_dir = config.root_dir.clone();
    let db_dir = root_dir.join("database");

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    setup_tracing(tracing_subscriber::registry(), config);

    Ok(())
}

pub fn setup_tracing<S>(subscriber: S, config: &AnythingEventsConfig)
where
    S: Subscriber + for<'a> LookupSpan<'a> + Send + Sync,
{
    global::set_text_map_propagator(TraceContextPropagator::new());

    let service_name = "event_service".to_string();

    match &config.tracing.otel_endpoint {
        None => setup_tracing_without_otel_collector(subscriber, service_name),
        Some(otel_endpoint) => {
            setup_tracing_with_otel_collector(subscriber, service_name, otel_endpoint.clone())
        }
    }
}

/// Setup tracing without otel collector
fn setup_tracing_without_otel_collector<S>(subscriber: S, _service_name: String)
where
    S: Subscriber + for<'a> LookupSpan<'a> + Send + Sync,
{
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = fmt::layer();

    subscriber.with(filter_layer).with(fmt_layer).init();
}

/// Setup tracing with otel collector
fn setup_tracing_with_otel_collector<S>(subscriber: S, service_name: String, otel_endpoint: String)
where
    S: Subscriber + for<'a> LookupSpan<'a> + Send + Sync,
{
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = fmt::layer();
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otel_endpoint),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                service_name.to_string(),
            )])),
        )
        .install_batch(Tokio)
        .unwrap();

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    subscriber
        .with(filter_layer)
        .with(fmt_layer)
        .with(otel_layer)
        .init();
}

/// Macro for instrumenting spans
#[macro_export]
macro_rules! instrumented {
    ($span:expr, $block:tt) => {{
        use tracing::Instrument;
        async {
            {
                $block
            };
            Ok::<(), anyhow::Error>(())
        }
        .instrument($span)
        .await
    }};
}