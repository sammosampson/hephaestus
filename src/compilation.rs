use std::{
    sync::{
        mpsc::*,
        Arc,
        Mutex
    },
    ops::DerefMut
};

use crate::{
    parsing::*,
    typing::*,
    threading::*
};

pub fn compile(file_name: String) {
    let mut job_queue = create_job_queue();
    enqueue_job(&mut job_queue, parse_file_job_request(file_name));
    process_jobs(job_queue);
}

fn process_jobs(mut job_queue: JobQueue) {
    loop {
        let jobs = process_job_results(&job_queue, drain_job_results(&job_queue));
        
        if jobs.len() == 0 && !are_any_jobs_active(&job_queue) {
            break;
        }
        
        enqueue_jobs(&mut job_queue, jobs);
    }
}

#[derive(Clone)]
pub enum JobResult {
    FileParsed(FileParseResult),
    UnitTyped(CompilationUnitId)
}

type JobResults = Vec<JobResult>;
pub type JobResultReceiver = Receiver<JobResult>;

fn process_job_results(job_queue: &JobQueue, results: JobResults) -> JobRequests {
    let mut jobs = vec!();

    for result in results {
        notify_all_jobs_of_result(job_queue, result.clone());

        match result {
            JobResult::FileParsed(result) => match result {
                FileParseResult::CompilationUnits { units, .. } => { 
                    process_parsed_compilation_units(units, &mut jobs);
                },
                FileParseResult::NotFound(file_name) => {
                    process_parse_file_not_found(file_name, &mut jobs);
                }
            }
            JobResult::UnitTyped(_) => todo!(),
        }       
    }

    jobs
}

fn process_parsed_compilation_units(units: CompilationUnits, jobs: &mut JobRequests) {
    for unit in units {
        jobs.push(perform_typing_job_request(unit))
    }
}

fn process_parse_file_not_found(file_name: String, _jobs: &mut JobRequests)  {
    panic!("{} not found", file_name);
}


struct JobQueue {
    pool: ThreadPool<JobResult>,
    sender: ConcurrentSender<JobResult>,
    receiver: JobResultReceiver
}

fn create_job_queue() -> JobQueue {
    let (sender, receiver) = channel::<JobResult>();

    JobQueue {
        pool: create_thread_pool(4),
        sender: get_concurrent_sender(sender),
        receiver
    }
}

fn notify_all_jobs_of_result(job_queue: &JobQueue, result: JobResult) {
    notify_all_running_tasks_of_result(&job_queue.pool, result);
}

fn are_any_jobs_active(job_queue: &JobQueue) -> bool {
    is_thread_pool_performing_work(&job_queue.pool)
}

fn drain_job_results(job_queue: &JobQueue) -> JobResults {
    let results = job_queue.receiver.try_iter();
    results.collect()
}

fn enqueue_jobs(job_queue: &mut JobQueue, jobs: JobRequests) {
    for job in jobs {
        enqueue_job(job_queue, job);
    }
}

fn enqueue_job(job_queue: &mut JobQueue, job: JobRequest) {    
    schedule_task(
        &mut job_queue.pool, 
        create_task(
            Box::new(move |mut runnable_receiver| handle_job_request(job.clone(), &mut runnable_receiver)), 
            clone_concurrent_sender(&job_queue.sender)
        )
    );
}

type JobRequest = Concurrent<JobRequestItem>;

enum JobRequestItem {
    ParseFile(String),
    PerformTyping(CompilationUnit),
}

type JobRequests = Vec<JobRequest>;

fn parse_file_job_request(file_name: String) -> JobRequest {
    create_concurrent(JobRequestItem::ParseFile(file_name))
}

fn perform_typing_job_request(unit: CompilationUnit) -> JobRequest {
    create_concurrent(JobRequestItem::PerformTyping(unit))
}

fn handle_job_request(job: JobRequest, runnable_receiver: &mut JobResultReceiver) -> JobResult {
    match lock(&job).deref_mut() {
        JobRequestItem::ParseFile(file_name) => JobResult::FileParsed(parse_file(file_name)),
        JobRequestItem::PerformTyping(unit) => JobResult::UnitTyped(perform_typing(unit, runnable_receiver)),
    }
}