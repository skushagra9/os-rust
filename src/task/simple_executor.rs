use std::{collections::VecDeque, task::{RawWaker, RawWakerVTable, Waker}};
use super::Task;
use core::task::{Poll, Context};

fn dummy_raw_worker() -> RawWaker{
    fn no_op(_: *const()){}
    fn clone(_: *const()) -> RawWaker{
        dummy_raw_worker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(0 as *const(), vtable)
}

fn dummy_waker() -> Waker{
    unsafe {
        Waker::from_raw(dummy_raw_worker())
    }
}

pub struct SimpleExecutor {
    tasks: VecDeque<Task>,
}

impl SimpleExecutor{
    pub fn new() -> SimpleExecutor{
        SimpleExecutor{
            tasks : VecDeque::new()
        }
    }

    pub fn spawn(&mut self, task: Task){
        self.tasks.push_back(task);
    }

    pub fn run(&mut self){
        while let Some(mut task) = self.tasks.pop_front(){
            let waker = dummy_waker();
            let mut context: Context<'_> = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Pending => self.tasks.push_back(task),
                Poll::Ready(()) => {}
            }
        }
    }
}