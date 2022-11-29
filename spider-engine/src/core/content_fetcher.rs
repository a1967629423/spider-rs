use super::content_resolver::ContentType;
#[async_trait::async_trait]
pub trait ContextID: std::any::Any {
    fn changed(&self) -> bool {
        true
    }
    async fn next_id(&mut self);
    /// 判断是否存在下一个ID，如果不存在任务结束
    fn has_next(&self) -> bool;
    fn as_any(&self) -> &dyn std::any::Any
    where
        Self: Sized,
    {
        self
    }
}
pub struct BoxContextID(Box<dyn ContextID + Send + Sync>);
#[derive(Debug, Default)]
pub struct EmptyContextID;

impl EmptyContextID {
    pub fn new() -> Self {
        Self
    }
}
#[async_trait::async_trait]
impl ContextID for EmptyContextID {
    fn has_next(&self) -> bool {
        true
    }
    async fn next_id(&mut self) {}
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
    fn next_id<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.0.next_id()
    }
}

pub struct BoxError(Box<dyn std::error::Error + Send>);

impl std::fmt::Display for BoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Debug for BoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
    fn name(&self) -> String {
        "anonymous fetcher".into()
    }

    async fn fetch_content(&mut self, id: &Self::ID) -> Result<Self::ContentType, Self::Error>;
}
