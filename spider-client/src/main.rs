use std::sync::Arc;

use anyhow::Result;
use async_provider::TokioAsyncProvider;
use spider_engine::core::{
    content_fetcher::EmptyContextID,
    content_resolver::LogContentResolver,
    schedule::TaskSchedule,
    task::{const_task_run_timer_type, ConstTaskRunTimer},
};
use tasks::BaiduHotSearchFetcher;
mod async_provider;
mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stdout);
    builder.filter_level(log::LevelFilter::Info);
    builder.init();
    let schedule = Arc::new(TaskSchedule::<TokioAsyncProvider>::new());
    schedule
        .add_box_task(
            BaiduHotSearchFetcher {},
            EmptyContextID::default(),
            LogContentResolver::default(),
            ConstTaskRunTimer::<60, { const_task_run_timer_type::SECS }>::default(),
        )
        .await;
    log::info!("start!!");
    let other_schedule = schedule.clone();
    other_schedule.schedule().await;
    Ok(())
}
