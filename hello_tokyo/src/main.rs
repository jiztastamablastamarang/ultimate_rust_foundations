async fn hello1() -> i32 {
    println!("Hello, tokyo1");
    return 1;
}

async fn hello2() -> i32 {
    println!("Hello, tokyo2");
    return 2;
}

/*
fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get())
        .build()
        .unwrap();

    rt.block_on(hello());
}
*/

async fn ticker() {
    for i in 0..5 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    /*    let (one, two) = tokio::join!(hello1(), hello2());
        println!("results: {:?}", (one, two));

        tokio::spawn(ticker());

        hello2().await;
    */
    let _ = tokio::join!(tokio::spawn(ticker()), tokio::spawn(ticker()),);
}
