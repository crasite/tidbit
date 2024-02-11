use axum::http::{HeaderMap, HeaderValue};
use opentelemetry::propagation::Extractor;

pub struct OtelHeader<'a> {
    pub headers: &'a HeaderMap<HeaderValue>,
}

impl<'a> From<&'a HeaderMap<HeaderValue>> for OtelHeader<'a> {
    fn from(headers: &'a HeaderMap<HeaderValue>) -> Self {
        OtelHeader { headers }
    }
}

impl<'a> Extractor for OtelHeader<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers.keys().map(|k| k.as_str()).collect()
    }
}
