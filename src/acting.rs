use std::sync::mpsc::*;
use crate::threading::*;

pub type ActorThreadPool = ThreadPool<()>;

pub fn create_actor_thread_pool() -> ActorThreadPool {
    create_thread_pool(40)
}

#[derive(Clone)]
pub struct ActorHandle<TMessage> {
    sender: Sender<TMessage>,
}

pub fn create_handle<TMessage>(sender: Sender<TMessage>) -> ActorHandle<TMessage>
where TMessage: ParallelisableClone {
    ActorHandle {
        sender
    }
}

pub enum AfterReceiveAction {
    ContinueListening,
    Shutdown
}

pub fn continue_listening_after_receive() -> AfterReceiveAction {
    AfterReceiveAction::ContinueListening
}

pub fn shutdown_after_receive() -> AfterReceiveAction {
    AfterReceiveAction::Shutdown
}

pub trait Actor<TMessage> : Parallelisable
    where TMessage: ParallelisableClone {
    fn receive(&mut self, message: TMessage, ctx: &ActorContext<TMessage>) -> AfterReceiveAction;
}

pub fn send_message_to_actor<TMessage>(actor: &ActorHandle<TMessage>, message: TMessage) {
    actor.sender.send(message).unwrap();
}

pub fn start_singleton_actor<TActor, TMessage>(actor: TActor) -> (ActorHandle<TMessage>, ActorShutdownNotifier)
where TActor: Actor<TMessage>, TMessage: ParallelisableClone {
    let thread_pool = create_actor_thread_pool();
    let concurrent_thread_pool = create_concurrent(thread_pool);
    start_actor_from_pool(concurrent_thread_pool, actor)
}

pub fn start_actor<TActor, TParentMessage, TMessage>(parent_context: &ActorContext<TParentMessage>, actor: TActor) -> (ActorHandle<TMessage>, ActorShutdownNotifier)
where TActor: Actor<TMessage>, TMessage: ParallelisableClone, TParentMessage: ParallelisableClone  {
    start_actor_from_pool(parent_context.thread_pool.clone(), actor)
}

fn start_actor_from_pool<TActor, TMessage>(pool: Concurrent<ActorThreadPool>, actor: TActor) -> (ActorHandle<TMessage>, ActorShutdownNotifier)
where TActor: Actor<TMessage>, TMessage: ParallelisableClone {
    let (sender, receiver) = channel::<TMessage>();
    let self_sender = sender.clone();
    
    let (task_sender, task_receiver) = channel::<()>();
    let concurrent_task_sender = get_concurrent_sender(task_sender);
    
    let pool_for_task = pool.clone();
    let context = create_context(self_sender, receiver, pool);
    let actor = create_actor(actor, context);
    
    schedule_task(
        pool_for_task.lock().as_ref().unwrap(), 
        create_task(
            Box::new(move || run_actor(actor)), 
            clone_concurrent_sender(&concurrent_task_sender)
        )
    );

    (create_handle(sender), create_shutdown_notifier(task_receiver))
}

pub struct ActorShutdownNotifier {
    receiver: Receiver<()>
}

fn create_shutdown_notifier(receiver: Receiver<()>) -> ActorShutdownNotifier {
    ActorShutdownNotifier { receiver } 
}

pub fn await_shutdown(shutdown_notifier: &ActorShutdownNotifier) {
    shutdown_notifier.receiver.recv().unwrap();
}

pub struct ActorContext<TMessage>
where TMessage: ParallelisableClone {
    self_sender: Sender<TMessage>,
    receiver: Receiver<TMessage>,
    thread_pool: Concurrent<ActorThreadPool>
}

fn create_context<TMessage>(self_sender: Sender<TMessage>, receiver: Receiver<TMessage>, thread_pool: Concurrent<ActorThreadPool>) -> ActorContext<TMessage>
where TMessage: ParallelisableClone {
    ActorContext {
        self_sender,
        receiver,
        thread_pool
    }
}

pub fn create_self_handle<TMessage>(context: &ActorContext<TMessage>) -> ActorHandle<TMessage>
where TMessage: ParallelisableClone {
    create_handle(context.self_sender.clone())
}

struct ActorRunner<TActor, TMessage>
where TActor: Actor<TMessage>, TMessage: ParallelisableClone {
    actor: TActor,
    context: ActorContext<TMessage>,
}

fn create_actor<TActor, TMessage>(actor: TActor, context: ActorContext<TMessage>) -> ActorRunner<TActor, TMessage>
where TActor: Actor<TMessage>, TMessage: ParallelisableClone {
    ActorRunner {
        actor,
        context
    }
}

fn run_actor<TActor, TMessage>(mut runner: ActorRunner<TActor, TMessage>)
where TActor: Actor<TMessage>, TMessage: ParallelisableClone {
    for message in &runner.context.receiver {
        if let AfterReceiveAction::Shutdown = runner.actor.receive(message, &runner.context) {
            break;
        }
    }
}

pub fn await_message<TMessage, TOnRecvMessage: FnMut(TMessage) -> bool>(context: &ActorContext<TMessage>, mut on_recv: TOnRecvMessage)
    where TMessage: ParallelisableClone {
        for message in &context.receiver {
            if on_recv(message) {
                break;
            }
        }
}
