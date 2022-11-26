use super::task::{BoxTask, TaskRunTimer};
use futures::channel::mpsc::{
    channel, unbounded, Receiver, Sender, UnboundedReceiver, UnboundedSender,
};
use futures::lock::Mutex;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::marker::PhantomData;

use std::sync::Arc;
use std::{collections::HashMap, sync::atomic::AtomicUsize};
pub trait AsyncExecutor {
    fn get_name() -> &'static str {
        "anonymous executor"
    }
    fn spawn(fu: futures::future::BoxFuture<'static, ()>);
    fn sleep(duration: std::time::Duration) -> futures::future::BoxFuture<'static, ()>;
}
#[derive(Debug,Clone)]
struct TimerScheduleData {
    task_id: usize,
    next_time: std::time::Duration,
}

pub struct TaskSchedule<E> {
    __mark: PhantomData<E>,
    current_id: AtomicUsize,
    task_fetchers: Arc<Mutex<HashMap<usize, BoxTask>>>,
    timer_schedule_sender: Mutex<UnboundedSender<TimerScheduleData>>,
    timer_schedule_receiver: Mutex<UnboundedReceiver<TimerScheduleData>>,
    end_schedule_sender: Mutex<Sender<()>>,
    end_schedule_receiver: Mutex<Receiver<()>>,
}
impl<E> TaskSchedule<E>
where
    E: AsyncExecutor,
{
    pub fn new() -> Self {
        let (timer_sender, timer_receiver) = unbounded();
        let (end_sender, end_receiver) = channel(10);
        Self {
            __mark: PhantomData::default(),
            current_id: AtomicUsize::new(0),
            task_fetchers: Arc::new(Mutex::new(HashMap::new())),
            timer_schedule_sender: Mutex::new(timer_sender),
            timer_schedule_receiver: Mutex::new(timer_receiver),
            end_schedule_sender: Mutex::new(end_sender),
            end_schedule_receiver: Mutex::new(end_receiver),
        }
    }
    pub async fn add_task(&self, mut task: BoxTask) -> usize {
        let id = self
            .current_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let next_time = {
            if task.can_run() {
                std::time::Duration::from_secs(0)
            } else {
                if let Some(t) = task.next_check() {
                    t
                } else {
                    return id;
                }
            }
        };
        self.task_fetchers.lock().await.insert(id, task);
        self.timer_schedule_sender
            .lock()
            .await
            .send(TimerScheduleData {
                task_id: id,
                next_time,
            })
            .await
            .ok();
        id
    }

    pub async fn schedule(&self) {
        let mut receiver = self.timer_schedule_receiver.lock().await;
        let mut end_receiver = self.end_schedule_receiver.lock().await;
        log::info!("start schedule async executor is {}", E::get_name());
        loop {
            futures::select! {
                timer_schedule_data =  receiver.next() => {
                    log::info!("receive timer data {:?}",timer_schedule_data);
                    if let Some(timer_schedule_data) = timer_schedule_data {
                        self.schedule_task(timer_schedule_data);
                    } else {
                        break;
                    }
                },
                _ = end_receiver.next() => {
                    log::info!("receive end signal,shutdown...");
                    break;
                }
            }
        }
    }
    pub async fn end_schedule(&self) {
        self.end_schedule_sender.lock().await.send(()).await.ok();
        E::sleep(std::time::Duration::from_secs(1)).await;
    }
    fn schedule_task(&self, mut data: TimerScheduleData) {
        let task_fetchers = Arc::downgrade(&self.task_fetchers);
        E::spawn(Box::pin(async move {
            loop {
                let duration = data.next_time;
                log::info!("task {} sleep {:?}",data.task_id,duration);
                if !duration.is_zero()   {

                    E::sleep(duration).await;
                }
                log::info!("task {} wake up",data.task_id);
                if let Some(task_fetchers) = task_fetchers.upgrade() {
                    let mut guard = task_fetchers.lock().await;
                    if let Some(task) = guard.get_mut(&data.task_id) {
                        if !task.can_run() {
                            break;
                        }
                        let task_run_start = std::time::Instant::now();
                        if let Some(Err(e)) = task.run_once().await {
                            log::error!("task {} run error {:?}",data.task_id,e);
                        }
                        log::info!("task {} run expend {:?}",data.task_id,std::time::Instant::now() - task_run_start);
                        if let Some(t) = task.next_check() {
                            data.next_time = t;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            if let Some(task_fetchers) = task_fetchers.upgrade() {
                task_fetchers.lock().await.remove(&data.task_id);
            }
        }))
    }
}
