use async_trait::async_trait;
use axum_core::extract::FromRequestParts;
use http::{request::Parts, HeaderMap};
use opentelemetry::{propagation::Extractor, Context};

pub use tracing::Span;
pub use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct ExtractOtelContext(pub Context);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractOtelContext
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&OtelHeadersExtractor(&parts.headers))
        });

        Ok(ExtractOtelContext(context))
    }
}

pub struct OtelHeadersExtractor<'a>(pub &'a HeaderMap);

impl<'a> Extractor for OtelHeadersExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}
