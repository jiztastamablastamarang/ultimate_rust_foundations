use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

async fn say_hello() {
    println!("Hello, world!");
    join!(say_hello_again(), say_goodbye());

    let n = double(10).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(3)];
    let results = join_all(futures).await;

    println!("{results:?}");
    
    no_async();
}

async fn say_hello_again() {
    println!("Hello, again!");
}

async fn say_goodbye() {
    println!("Goodbye!");
}

async fn double(n: u32) -> u32 {
    return n * 2;
}

fn no_async() {
    println!("Hello, sync world!");    
}

fn main() {
    block_on(say_hello());
}
