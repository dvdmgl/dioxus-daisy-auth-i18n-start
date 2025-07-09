use std::sync::LazyLock;

use opentelemetry::{KeyValue, trace::TracerProvider};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace::Sampler, trace::SdkTracerProvider};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    EnvFilter,
    {layer::SubscriberExt, util::SubscriberInitExt},
};

static RESOURCE: LazyLock<Resource> = LazyLock::new(|| {
    Resource::builder()
        .with_attribute(KeyValue::new("service.name", "dioxus-server"))
        .build()
});

pub fn init_tracer(endpoint: &str) -> SdkTracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
        .expect("Failed to build span exporter");

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(RESOURCE.clone())
        // Sampler::TraceIdRatioBased for production
        .with_sampler(Sampler::AlwaysOn)
        .build();

    let otel_layer = OpenTelemetryLayer::new(provider.tracer("dioxus-app"));

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,tower_http=debug,dioxus_fullstack=debug")); // Default filter

    tracing_subscriber::registry()
        .with(otel_layer)
        // .with(tracing_subscriber::fmt::layer()) // Add a formatter for console output
        .with(env_filter)
        .init();
    provider
}
