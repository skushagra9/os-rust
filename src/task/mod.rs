use core::{future::Future, pin::Pin};
use core::sync::atomic::{AtomicU64, Ordering};

pub mod simple_executor;
pub mod keyboard;
pub mod executor;

use core::task::{Context};
use std::task::Poll;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    id: TaskId,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
            id: TaskId::new(),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
        
    }
    
}