use super::content_fetcher::BoxContentType;
use super::content_fetcher::BoxContextID;
use super::content_fetcher::BoxError;
use super::content_fetcher::ContentFetcher;
use super::content_fetcher::ContextID;
use super::content_resolver::ContentResolver;
pub trait TaskRunTimer {
    fn next_check(&mut self) -> Option<std::time::Duration>;
    fn can_run(&self) -> bool;
}
pub mod const_task_run_timer_type {
    pub const SECS:u8 = 0;
    pub const MILLIS:u8 = 1;
    pub const MICROS:u8 = 2;
}
#[derive(Debug,Default)]
pub struct ConstTaskRunTimer<const DURATION:u64,const TIME_TYPE:u8>;
impl<const DURATION:u64,const TIME_TYPE:u8>  ConstTaskRunTimer<DURATION,TIME_TYPE> {
    pub fn new() -> Self {
        Self
    }
}
impl<const DURATION:u64,const TIME_TYPE:u8> TaskRunTimer for ConstTaskRunTimer<DURATION,TIME_TYPE> {
    fn can_run(&self) -> bool {
        true
    }
    fn next_check(&mut self) -> Option<std::time::Duration> {

        let duration = match TIME_TYPE {
            0 => {
                std::time::Duration::from_secs(DURATION)
            },
            1 => {
                std::time::Duration::from_millis(DURATION)
            },
            2 => {
                std::time::Duration::from_micros(DURATION)
            }
            _ => {
                std::time::Duration::from_secs(DURATION)
            }
        };
        Some(duration)
    }
}


pub struct BoxTask {
    fetcher: Box<
        dyn ContentFetcher<
            ContentType = BoxContentType,
            ID = BoxContextID,
            Error = BoxError,
        > + Send,
    >,
    resolver: Box<dyn ContentResolver<Error = BoxError, ContentType = BoxContentType> + Send>,
    timer: Box<dyn TaskRunTimer + Send>,
    id: BoxContextID,
}

impl BoxTask {
    pub fn new<F, ID, R, T>(fetcher: F, id: ID, resolver: R, timer: T) -> Self
    where
        F: ContentFetcher<
                Error = BoxError,
                ContentType = BoxContentType,
                ID = BoxContextID,
            > + Send +  'static,
        R: ContentResolver<Error = BoxError, ContentType = F::ContentType> + Send + 'static,
        T: TaskRunTimer + Send + 'static,
        ID: ContextID + Send + Sync + 'static,
    {
        Self {
            fetcher: Box::new(fetcher),
            resolver: Box::new(resolver),
            timer: Box::new(timer),
            id: BoxContextID::new(id),
        }
    }
    pub async fn run_once(&mut self) -> Option<Result<(), BoxError>> {
        if !self.id.has_next() {
            return None;
        }
        let fu = self.fetcher.fetch_content(&self.id);
        let result = fu.await;
        match result {
            Ok(result) => {
                let resolve_content = self.resolver.resolve_content(result).await;
                if let Err(e) = resolve_content {
                    return Some(Err(e));
                }
                return Some(Ok(()));
            }
            Err(e) => return Some(Err(e)),
        }
    }
}

impl TaskRunTimer for BoxTask {
    fn can_run(&self) -> bool {
        self.timer.can_run()
    }
    fn next_check(&mut self) -> Option<std::time::Duration> {
        self.timer.next_check()
    }
}
