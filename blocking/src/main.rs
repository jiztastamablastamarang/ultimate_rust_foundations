use std::time::Duration;
use tokio::task::spawn_blocking;

async fn delay(task: u64, time: u64) {
    println!("task {task} start");
    let _ = spawn_blocking(move || tokio::time::sleep(Duration::from_millis(time))).await;

    println!("task {task} end");
}

#[tokio::main]
async fn main() {
    tokio::join!(delay(1, 1000), delay(2, 2000), delay(3, 3000));
}
