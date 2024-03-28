fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello, world!"));
    pool.scope(|s| {
        for i in 0..10 {
            s.spawn(move |_| println!("Hello from the scope {i}"))
        }
    });
    /*
      pool.scope(|s| {
        s.spawn_broadcast(|_, ctx| {
            println!("Hello from broadcast thread {}", ctx.index());
        })
    });
    */

    pool.join(test, test);
}

fn test() {
    println!("Hello, world!")
}
