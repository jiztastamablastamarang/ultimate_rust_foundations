fn hello_thread(n: u32) {
    println!("Hello, from the {n}-th thread!");
}

fn do_math(i: u32) -> u32 {
    let mut n = i + 1;
    for _ in 0..10 { n *= 2 }

    return n;
}

fn main() {
    println!("Hello, from the main thread!");

    let mut thread_handles = Vec::new();
    for i in 0..10 {
        thread_handles.push(std::thread::spawn(move || do_math(i)))
    }

    thread_handles.into_iter().for_each(|h| { println!("Joining thread {}", h.join().unwrap()) });
}
