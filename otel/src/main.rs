use opentelemetry::{propagation::TextMapPropagator as _, trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use otel_header::OtelHeader;
use std::collections::HashMap;

use tracing::{error, info, span, Instrument, Level};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};
mod otel_header;

use opentelemetry::global;
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    runtime,
    trace::{self, BatchSpanProcessor, TracerProvider},
    Resource,
};

use axum::{
    body::Body,
    http::Request,
    middleware::{from_fn, Next},
    response::Response,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    init_logger();
    global::set_text_map_propagator(TraceContextPropagator::new());
    let app = Router::new()
        .route("/", get(index))
        .layer(from_fn(otel_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3008").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    global::shutdown_tracer_provider();
}

#[tracing::instrument]
async fn index() -> String {
    info!("Hello, world!");
    tracing::trace!("This is a trace");
    let mut x = HashMap::new();
    {
        let _child = span!(tracing::Level::INFO, "child", foo = 1).entered();
        let ctx = tracing::Span::current().context();
        TraceContextPropagator::new().inject_context(&ctx, &mut x);
        error!("This is an error");
    }
    format!("Hello, world!\nContext: {:?}", x)
}

async fn otel_middleware(request: Request<Body>, next: Next) -> Response {
    let x: OtelHeader = request.headers().into();
    let parent_context = global::get_text_map_propagator(|propagator| propagator.extract(&x));
    let span = span!(tracing::Level::INFO, "incoming_request");
    span.set_parent(parent_context);
    async { next.run(request).await }.instrument(span).await
}

fn init_logger() {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://tempo:4317")
        .build_span_exporter()
        .unwrap();
    let processor = BatchSpanProcessor::builder(exporter, runtime::Tokio).build();
    let provider = TracerProvider::builder()
        .with_span_processor(processor)
        .with_config(
            trace::config().with_resource(Resource::new([KeyValue::new("service.name", "otlp")])),
        )
        .build();
    global::set_tracer_provider(provider.clone()); // set the global tracer provider, not really needed
    let tracer = provider.tracer("otel");
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let filter = tracing_subscriber::filter::Targets::new().with_target("otel", Level::TRACE);
    let otel_filter = tracing_subscriber::filter::Targets::new().with_target("otel", Level::INFO);
    Registry::default()
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .with(opentelemetry.with_filter(otel_filter))
        .init();
}
