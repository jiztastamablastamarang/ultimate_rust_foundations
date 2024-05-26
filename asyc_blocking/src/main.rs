use std::time::Duration;

#[tokio::main]
async fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    println!("Sum: {}", parallel_sum(nums).await);
}

async fn parallel_sum(nums: Vec<i32>) -> i32 {
    let (tx, rx) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        // Do some EXPENSIVE computation.
        let mut sum = 0;
        for num in nums {
            sum += num;
        }
       // send to tokio. 
        let _ = tx.send(sum);
    });

    // Wait for the rayon task.
    rx.await.expect("failed to receive result on channel")
}

async fn main_blocking() {
    let blocking_task = tokio::task::spawn_blocking(|| {
        println!("Hello spawn blocking");
    });

    blocking_task.await.unwrap();
}

async fn sleep_then_print(timer: i32) {
    println!("Start timer: {timer}");

    tokio::time::sleep(Duration::from_secs(1));
    println!("End timer: {timer}");
}
/*
#[tokio::main]
async fn main() {
    tokio::join!(
        sleep_then_print(1),
        sleep_then_print(2),
        sleep_then_print(3),
    );
}
async fn sleep_then_print(timer: i32) {
    println!("Start timer: {timer}");

    // Blocks thread
    std::thread::sleep(Duration::from_secs(1));
    println!("End timer: {timer}");
}
*/
/*#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // Blocks thread
    std::thread::sleep(Duration::from_sec(5));

    println!("Hello, latter world");
}
*/
