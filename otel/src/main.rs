use opentelemetry::trace::{Span, Tracer, TracerProvider as _};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{error, error_span, info, span, Instrument, Level};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

use opentelemetry::global;
use opentelemetry_sdk::{
    runtime,
    trace::{BatchSpanProcessor, TracerProvider},
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
    let _guard = init_logger();
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(index))
        // .layer(TraceLayer::new_for_http());
        .layer(from_fn(otel_middleware));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    global::shutdown_tracer_provider();
}

#[tracing::instrument]
async fn index() -> &'static str {
    info!("Hello, world!");
    {
        let _child = span!(tracing::Level::TRACE, "child", foo = 1).entered();
        error!("This event will be logged in the child span.");
    }
    "Hello, world!"
}

// #[axum_macros::debug_handler]
async fn otel_middleware(request: Request<Body>, next: Next) -> Response {
    // do something with `request`...

    let span = span!(tracing::Level::TRACE, "incoming_request", work_units = 2);
    async { next.run(request).await }.instrument(span).await
}

fn init_logger() {
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    // let tracer = opentelemetry_jaeger::new_agent_pipeline()
    //     .with_service_name("public")
    //     .install_simple()
    //     .unwrap();
    let processor = BatchSpanProcessor::builder(
        opentelemetry_stdout::SpanExporter::default(),
        runtime::Tokio,
    )
    .build();
    let provider = TracerProvider::builder()
        .with_span_processor(processor)
        .build();
    global::set_tracer_provider(provider.clone());
    let tracer = provider.tracer("otel");
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("otel", Level::TRACE)
        .with_target("tower_http", Level::TRACE);
    Registry::default()
        .with(tracing_subscriber::fmt::layer().with_filter(filter.clone()))
        .with(opentelemetry.with_filter(filter))
        .init();
}
