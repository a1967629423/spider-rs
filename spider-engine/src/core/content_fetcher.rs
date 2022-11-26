use super::content_resolver::ContentType;

pub trait ContextID {
    fn next_id(&mut self);
    fn has_next(&self) -> bool;
}
pub struct BoxContextID(Box<dyn ContextID + Send + Sync>);
#[derive(Debug, Default)]
pub struct EmptyContextID;

impl EmptyContextID {
    pub fn new() -> Self {
        Self
    }
}

impl ContextID for EmptyContextID {
    fn has_next(&self) -> bool {
        true
    }
    fn next_id(&mut self) {}
}

impl BoxContextID {
    pub fn new<T>(id: T) -> Self
    where
        T: ContextID + Send + Sync + 'static,
    {
        BoxContextID(Box::new(id))
    }
}

impl ContextID for BoxContextID {
    fn has_next(&self) -> bool {
        self.0.has_next()
    }
    fn next_id(&mut self) {
        self.0.next_id()
    }
}
#[derive(Debug)]
pub struct BoxError(Box<dyn std::error::Error + Send>);

impl std::fmt::Display for BoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BoxError").field(&self.0).finish()
    }
}
impl std::error::Error for BoxError {}

impl BoxError {
    pub fn new<T>(err: T) -> Self
    where
        T: std::error::Error + Send + 'static,
    {
        Self(Box::new(err))
    }
    pub fn from_send_sync_box(err: Box<dyn std::error::Error + Sync + Send + 'static>) -> Self {
        Self(err)
    }
}

pub struct BoxContentType(Box<dyn ContentType + Sync + Send + 'static>);
impl BoxContentType {
    pub fn new<T>(content: T) -> Self
    where
        T: ContentType + Sync + Send + 'static,
    {
        Self(Box::new(content))
    }
    pub fn into_inner(self) -> Box<dyn ContentType + Sync + Send + 'static> {
        self.0
    }
}
impl ContentType for BoxContentType {
    fn custom_debug(&self) -> Option<Box<dyn std::fmt::Debug>> {
        self.0.custom_debug()
    }
}
#[async_trait::async_trait]
pub trait ContentFetcher {
    type ContentType;
    type ID: ContextID;
    type Error: std::error::Error;

    async fn fetch_content(&mut self, id: &Self::ID) -> Result<Self::ContentType, Self::Error>;
}
