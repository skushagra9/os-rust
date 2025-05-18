pub mod task;
use task::{simple_executor::{SimpleExecutor}, Task};

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


fn main(){
    let mut executor = SimpleExecutor::new();
    let task = Task::new(example_task());
    executor.spawn(task);
    executor.run();
}