use std::{borrow::BorrowMut, mem::take};

struct QueueNode<T> {
    value: T,
    next: Option<Box<QueueNode<T>>>,
}

impl<T> QueueNode<T> {
    fn new(value: T) -> QueueNode<T> {
        QueueNode { value, next: None }
    }
}

pub struct Queue<T> {
    end: Option<QueueNode<T>>,
}

pub fn create_queue<T>() -> Queue<T> {
    Queue { end: None }
}

pub fn enqueue<T>(queue: &mut Queue<T>, value: T) {
    let new_node = QueueNode::new(value);
    if let Some(end) = &mut queue.end {
        let mut start = end;
        loop {
            if let Some(_) = &start.next {
                start = (start.next.as_mut().unwrap()).borrow_mut();
            } else {
                break;
            }
        }
        start.next = Some(Box::new(new_node));
    } else {
        queue.end = Some(new_node);
    }
}

pub fn dequeue<T>(queue: &mut Queue<T>) -> Option<T> {
    if !is_queue_empty(queue) {
        let end = take(&mut queue.end).unwrap();
        if let Some(next) = end.next {
            queue.end = Some(*next);
        }
        Some(end.value)
    } else {
        None
    }
}

fn is_queue_empty<T>(queue: &Queue<T>) -> bool {
    match queue.end {
        None => true,
        _ => false,
    }
}