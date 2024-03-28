fn parkable_thread(n: u32) {
    loop {
        std::thread::park();
        println!("Thread {n} unparked!");
    }
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        threads.push(std::thread::spawn(move || {
            parkable_thread(i);
        }));
    }
}
