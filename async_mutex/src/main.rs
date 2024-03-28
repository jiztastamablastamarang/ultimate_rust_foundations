use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

async fn add_one(n: u32) -> u32 {
    return n + 1;
}

async fn increment() {
    let mut counter = COUNTER.lock().await;
    *counter = add_one(*counter).await;
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment());
    println!("Counter = {:?}", COUNTER.lock().await);
}
