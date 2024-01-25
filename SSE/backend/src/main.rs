use async_stream::stream;
use axum::http::HeaderValue;
use axum::http::Method;
use std::{convert::Infallible, time::Duration};
use tower_http::cors::CorsLayer;

use axum::Router;
use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::get,
};
use futures_util::stream::Stream;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/stream", get(sse_handler));

    let app = app.layer(create_cors_layer());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let s = stream! {
        yield Ok(Event::default().data("Starting Request"));
        tracing::info!("Starting Request");
        tokio::time::sleep(Duration::from_secs(3)).await;
        yield Ok(Event::default().data("Done Part 1"));
        tracing::info!("Done Part 1");
        tokio::time::sleep(Duration::from_secs(3)).await;
        yield Ok(Event::default().data("Done Part 2"));
        tracing::info!("Done Part 2");
        tokio::time::sleep(Duration::from_secs(3)).await;
        yield Ok(Event::default().data("Done"));
        tracing::info!("Done");
    };

    Sse::new(s).keep_alive(KeepAlive::default())
}

fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin("http://localhost:4321".parse::<HeaderValue>().unwrap())
}
