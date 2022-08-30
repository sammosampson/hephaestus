
mod tokenisation;
mod abstract_syntax;
mod source_files;
mod operators;
mod literals;
mod keywords;
mod ranges;
mod enclosures;
mod terminators;
mod directives;
mod file_system;
mod arguments;
mod types;
mod testing;
mod threading;
mod collections;

use abstract_syntax::*;
use threading::*;
use std::sync::mpsc::*;


fn main() {
    match arguments::get_file_to_compile_from_invocation_arguments() {
        Some(file_name) => { compile(file_name); },
        None => panic!("No compilation file name argument passed")
    }
}

fn compile(file_name: String) {
    let mut job_queue = create_job_queue();
    enqueue_job(&mut job_queue, parse_file_job_request(file_name));
    process_jobs(job_queue);
}

fn process_jobs(mut job_queue: JobQueue) {
    loop {
        match process_job_results(drain_job_results(&job_queue)) {
            Some(jobs) => {
                enqueue_jobs(&mut job_queue, jobs);
            },
            None => {
                if !are_any_jobs_active(&job_queue) {
                    break;
                }
            }
        }
    }
}

enum JobResult {
    FileParsed(AbstractSyntaxTree),
}

fn process_job_results(results: Vec<JobResult>) -> Option<Vec<JobRequest>> {
    for result in results {
        match result {
            JobResult::FileParsed(tree) => { dbg!(tree); },
        }
    }
    None
}

struct JobQueue {
    pool: ThreadPool<JobResult>,
    sender: ConcurrentSender<JobResult>,
    receiver: Receiver<JobResult>
}

fn create_job_queue() -> JobQueue {
    let (sender, receiver) = channel::<JobResult>();

    JobQueue {
        pool: create_thread_pool(4),
        sender: get_concurrent_sender(sender),
        receiver
    }
}

fn are_any_jobs_active(job_queue: &JobQueue) -> bool {
    is_thread_pool_performing_work(&job_queue.pool)
}

fn drain_job_results(job_queue: &JobQueue) -> Vec<JobResult> {
    let results = job_queue.receiver.try_iter();
    results.collect()
}


fn enqueue_jobs(job_queue: &mut JobQueue, jobs: Vec<JobRequest>) {
    for job in jobs {
        enqueue_job(job_queue, job);
    }
}

fn enqueue_job(job_queue: &mut JobQueue, job: JobRequest) {    
    schedule_task(
        &mut job_queue.pool, 
        create_task(Box::new(move || handle_job_request(job.clone())), clone_concurrent_sender(&job_queue.sender)));
}

#[derive(Clone)]
enum JobRequest {
    ParseFile(String),
}

fn parse_file_job_request(file_name: String) -> JobRequest {
    JobRequest::ParseFile(file_name)
}

fn handle_job_request(job: JobRequest) -> JobResult {
    match job {
        JobRequest::ParseFile(file_name) => JobResult::FileParsed(parse_file(&file_name)),
    }
}