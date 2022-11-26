use super::content_fetcher::{BoxContentType, BoxError};

pub trait ContentType: std::any::Any {
    fn custom_debug(&self) -> Option<Box<dyn std::fmt::Debug>> {
        None
    }
}

#[async_trait::async_trait]
pub trait ContentResolver {
    type ContentType: ContentType;
    type Error: std::error::Error;

    async fn resolve_content(&mut self, content: Self::ContentType) -> Result<(), Self::Error>;
}
#[derive(Debug, Default)]
pub struct LogContentResolver;

impl LogContentResolver {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ContentResolver for LogContentResolver {
    type ContentType = BoxContentType;
    type Error = BoxError;

    async fn resolve_content(&mut self, content: Self::ContentType) -> Result<(), Self::Error> {
        let inner = content.into_inner();
        if let Some(d) = inner.custom_debug() {
            log::info!("resolve: {:?}", d.as_ref())
        } else {
            log::info!("resolve: unknown");
        }
        Ok(())
    }
}
