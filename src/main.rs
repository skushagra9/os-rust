#![feature(abi_x86_interrupt)]

pub mod task;
use task::{executor::Executor, Task};
use std::time::Duration;
pub mod interrupts;
pub mod pic;

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


fn main(){

    // let mut executor = SimpleExecutor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(task::keyboard::print_keypresses()));
    // executor.run();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(task::keyboard::print_keypresses()));
    executor.run();

    // let duration = Duration::from_secs(10000);
    // std::thread::sleep(duration);
}