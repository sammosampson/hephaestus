use std::sync::atomic::*;
use std::sync::mpsc::*;
use std::sync::*;
use std::thread;
use crate::collections::*;

pub type Lockable<T> = Mutex<T>;
pub type Locked<'a, T> = MutexGuard<'a, T>;
pub type Shareable<T> = Arc<T>;
pub type Concurrent<T> = Shareable<Lockable<T>>;
pub type Notifier = Sender<()>;
pub type NotificationReceiver = Receiver<()>;
pub type ConcurrentSender<T> = Concurrent<Sender<T>>;
type ConcurrentBool = Shareable<AtomicBool>;

pub fn lock<T>(to_lock: &Lockable<T>) -> Locked<T> {
    to_lock.lock().unwrap()
}

fn create_lockable<T>(to_wrap: T) -> Lockable<T> {
    Mutex::new(to_wrap)
}

pub fn create_sharable<T>(to_wrap: T) -> Shareable<T> {
    Arc::new(to_wrap)
}

pub fn clone_shareable<T>(sender_shared: &Shareable<T>) -> Shareable<T> {
    Arc::clone(sender_shared)
}

pub fn create_concurrent<T>(to_wrap: T) -> Concurrent<T> {
    create_sharable(create_lockable(to_wrap))
}

fn send<T>(sender: &Sender<T>, to_send: T) {
    if let Err(error) = sender.send(to_send) {
        println!("error sending message in threading: {}", error)
    }
}

fn notify(sender: &Notifier) {
    send(sender, ());
}

pub fn clone_concurrent_sender<T>(sender_shared: &ConcurrentSender<T>) -> ConcurrentSender<T> {
    clone_shareable(sender_shared)
}

pub fn get_concurrent_sender<T>(sender: Sender<T>) -> ConcurrentSender<T> {
    create_concurrent(sender)
}

pub fn create_concurrent_bool() -> ConcurrentBool {
    create_sharable(AtomicBool::new(false))
}

pub fn compare_and_exchange_concurrent_bool_value(to_compare_and_exchange: &ConcurrentBool, current: bool, new: bool) -> Result<bool, bool> {
    to_compare_and_exchange.compare_exchange(current, new, Ordering::Acquire, Ordering::Relaxed)
}

pub fn set_concurrent_bool(to_set: &ConcurrentBool, to: bool) {
    to_set.store(to, Ordering::Relaxed)
}


pub trait ParallelisableClone: Parallelisable + Clone {
}

pub trait Parallelisable: Send + 'static {
}

impl<T> Parallelisable for T 
    where T: Send + 'static {
}

impl<T> ParallelisableClone for T 
    where T: Parallelisable + Clone {
}

pub type ParallelisableRunner<T> = Box<dyn FnOnce() -> T + Send>;
type LockableTaskMessageSender<T> = Lockable<Sender<Task<T>>>;
type Workers<T> = Vec<Worker<T>>;
type ShareableWorkers<T> = Shareable<Workers<T>>;
type ConcurrentTasks<T> = Concurrent<Tasks<T>>;
type Tasks<T> = Queue<Task<T>>;

pub struct ThreadPool<T: ParallelisableClone> {
    workers: ShareableWorkers<T>, 
    tasks: ConcurrentTasks<T>
}

pub fn schedule_task<T: ParallelisableClone>(thread_pool: &ThreadPool<T>, task: Task<T>) {
    if let Some(worker) = find_and_acquire_free_worker(&thread_pool.workers) {            
        send_task_to_worker(worker, task);
    } else {
        enqueue(&mut lock(&thread_pool.tasks), task);
    }
}

pub fn create_thread_pool<T: ParallelisableClone>(number_of_workers: u8) -> ThreadPool<T> {
    let (
        worker_free_notifier, 
        worker_free_notification_receiver
    ) = channel();
    
    let workers = create_workers(
        number_of_workers,
        worker_free_notifier
    );

    let tasks = create_concurrent(create_queue::<Task<T>>());

    start_schedule_tasks_thread(
        worker_free_notification_receiver, 
        clone_shareable(&tasks),
        clone_shareable(&workers)
    );

    ThreadPool::<T> {
        workers,
        tasks
    }
}

fn start_schedule_tasks_thread<T: ParallelisableClone>(
    worker_free_notification_receiver: NotificationReceiver, 
    tasks: ConcurrentTasks<T>,
    workers: ShareableWorkers<T>
) {
    thread::spawn(move || {            
        for _ in worker_free_notification_receiver {                
            if let Some(task) = dequeue(&mut lock(&tasks)) {
                if let Some(worker) = find_and_acquire_free_worker(&workers) {
                    send_task_to_worker(worker, task);
                }
            }
        }
    });
}



pub struct Task<T: ParallelisableClone> {
    runnable: ParallelisableRunner<T>,
    result_sender: ConcurrentSender<T>,
}

pub fn create_task<T: ParallelisableClone>(runnable: ParallelisableRunner<T>, result_sender: ConcurrentSender<T>) -> Task<T> {
    Task {
        runnable,
        result_sender,
    }
}

struct Worker<T: ParallelisableClone> {
    task_sender: LockableTaskMessageSender<T>, 
    is_running: ConcurrentBool,
}

fn send_task_to_worker<T: ParallelisableClone>(worker: &Worker<T>, task: Task<T>) {
    if let Err(e) = lock(&worker.task_sender).send(task) {
        println!("{}", e);
    }
}


fn create_workers<T: ParallelisableClone>(number_of_workers: u8, worker_free_notifier: Notifier) -> ShareableWorkers<T> {
    let mut workers = vec![];

    for _ in 0..number_of_workers {
        workers.push(create_worker(worker_free_notifier.clone()));
    }

    create_sharable(workers)
}

fn create_worker<T: ParallelisableClone>(worker_free_notifier: Notifier) -> Worker<T> {
    let (worker_task_sender, worker_task_receiver) = channel::<Task<T>>();
    let is_running = create_concurrent_bool();
    let shared_is_running = is_running.clone();    

    thread::spawn(move || {
        for task in worker_task_receiver {                
            send(&lock(&task.result_sender), (task.runnable)()); 
            set_concurrent_bool(&clone_shareable(&shared_is_running), false);
            notify(&worker_free_notifier);
        }
    });

    Worker {
        task_sender: create_lockable(worker_task_sender),
        is_running,
    }
}

fn find_and_acquire_free_worker<T: ParallelisableClone>(workers: &Workers<T>) -> Option<&Worker<T>> {
    workers.iter().find(|worker| acquire_free_worker(*worker))
}

fn acquire_free_worker<T: ParallelisableClone>(worker: &Worker<T>) -> bool {
    match compare_and_exchange_concurrent_bool_value(&worker.is_running, false, true) {
        Ok(_) => true,
        Err(_) => false,
    }
}


