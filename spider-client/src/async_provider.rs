use spider_engine::core::schedule::AsyncExecutor;


#[derive(Debug)]
pub struct TokioAsyncProvider;


impl AsyncExecutor for TokioAsyncProvider {
    fn get_name() -> &'static str {
        "tokio executor"
    }
    fn spawn(fu: futures::future::BoxFuture<'static, ()>) {
        tokio::spawn(fu);
    }
    fn sleep(duration: std::time::Duration) -> futures::future::BoxFuture<'static, ()> {
        Box::pin(async move {
            tokio::time::sleep(duration).await;
        })
    }
}