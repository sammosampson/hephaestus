use std::sync::atomic::*;
use std::sync::mpsc::*;
use std::sync::*;
use std::thread;

use crate::collections::*;

type Runnable<T> = Box<dyn Fn() -> T + Send>;
pub type ConcurrentSender<T> = Arc<Mutex<Sender<T>>>;
type ConcurrentAtomicBool = Arc<AtomicBool>;
type SenderIn<T> = Mutex<Sender<Task<T>>>;
type Workers<T> = Vec<Worker<T>>;
type ConcurrentWorkers<T> = Arc<Workers<T>>;
type Tasks<T> = Queue<Task<T>>;
type ConcurrentTasks<T> = Arc<Mutex<Tasks<T>>>;


pub struct Task<T: Send> {
    runnable: Runnable<T>,
    result_sender: ConcurrentSender<T>,
}

pub fn create_task<T: Send>(runnable: Runnable<T>, result_sender: ConcurrentSender<T>) -> Task<T> {
    Task {
        runnable,
        result_sender,
    }
}

pub struct Worker<T: Send> {
    sender_in: SenderIn<T>, 
    is_running: ConcurrentAtomicBool,
}

fn send_task_to_worker<T: Send + 'static>(worker: &Worker<T>, task: Task<T>) {
    if let Err(e) = worker.sender_in.lock().unwrap().send(task) {
        println!("{}", e);
    }
}

fn create_worker<T: Send + 'static>(sender_out: Sender<()>) -> Worker<T> {
    let (sender_in, receiver) = channel::<Task<T>>();
    let is_running = Arc::new(AtomicBool::new(false));
    let is_running_clone = Arc::clone(&is_running);
    
    thread::spawn(move || {
        for task in receiver {                
            task.result_sender
                .lock()
                .unwrap()
                .send((task.runnable)())
                .unwrap(); 
            is_running_clone.store(false, Ordering::Relaxed);
            sender_out.send(()).unwrap();
        }
    });

    Worker {
        sender_in: Mutex::new(sender_in),
        is_running,
    }
}

pub fn find_free_worker<T: Send>(workers: &Workers<T>) -> Option<&Worker<T>> {
    workers.iter().find(|worker| is_worker_free(*worker))
}

pub fn are_all_workers_free<T: Send>(workers: &Workers<T>) -> bool {
    workers.iter().all(|worker| is_worker_free(worker))
}

fn is_worker_free<T:Send>(worker: &Worker<T>) -> bool {
    match worker
        .is_running
        .compare_exchange(false, true,
           Ordering::Acquire, Ordering::Relaxed)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub struct ThreadPool<T: Send> {
    workers: ConcurrentWorkers<T>, 
    tasks: ConcurrentTasks<T>
}

pub fn schedule_task<T: Send + 'static>(thread_pool: &mut ThreadPool<T>, task: Task<T>) {
    if let Some(worker) = find_free_worker(&thread_pool.workers) {            
        send_task_to_worker(worker, task);
    } else {
        let mut queue = thread_pool.tasks.lock().unwrap();
        enqueue(&mut queue, task);
    }
}

pub fn create_thread_pool<T: Send + 'static>(n_workers: u8) -> ThreadPool<T> {
    let mut workers = vec![];
    let (sender, receiver) = channel();
    let tasks = Arc::new(Mutex::new(create_queue::<Task<T>>()));

    for _ in 0..n_workers {
        let sender_clone = sender.clone();
        workers.push(create_worker(sender_clone));
    }
    let workers = Arc::new(workers);
    let tasks_copy = Arc::clone(&tasks);
    let workers_copy = Arc::clone(&workers);

    thread::spawn(move || {            
        for _ in receiver {                
            let mut queue = tasks_copy.lock().unwrap();                
            if let Some(task) = dequeue(&mut queue) {
                if let Some(worker) = find_free_worker(&workers_copy) {
                    send_task_to_worker(worker, task);
                }
            }
        }
    });

    ThreadPool { workers, tasks }
}

pub fn is_thread_pool_performing_work<T:Send>(thread_pool: &ThreadPool<T>) -> bool {
    let workers_copy = Arc::clone(&thread_pool.workers);
    !are_all_workers_free(&workers_copy)
}

pub fn clone_concurrent_sender<T>(sender_shared: &ConcurrentSender<T>) -> ConcurrentSender<T> {
    Arc::clone(sender_shared)
}

pub fn get_concurrent_sender<T>(sender: Sender<T>) -> ConcurrentSender<T> {
    Arc::new(Mutex::new(sender))
}


