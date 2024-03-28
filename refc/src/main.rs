use std::sync::{Arc, Mutex};

struct Shared {
    data: Mutex<String>, // Interior mutability.
}

impl Shared {
    fn new(data: &str) -> Self {
        return Self {
            data: Mutex::new(data.to_string()),
        };
    }
}

fn main() {
    let shared = Arc::new(Shared::new("Hello"));

    let mut threads = Vec::new();

    for i in 0..10 {
        let shared = shared.clone();
        let thread = std::thread::spawn(move || {
            let mut data = shared.data.lock().unwrap();
            data.push_str(&format!(" {i}"));
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let data = shared.data.lock().unwrap();
    println!("{}", data);
}
