#[async_recursion::async_recursion]
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

async fn two() {
    println!("2");
}

async fn call_some(n: u32) {
   match n {
       1 => one().await,
       2 => two().await,
       _ => (),
   } 
}

#[tokio::main]
async fn main() {
    let mut future = async {
        println!("{}", fibonacci(10).await);
    };

    tokio::pin!(future);
    (&mut future).await;
    
    call_some(2).await;
}
