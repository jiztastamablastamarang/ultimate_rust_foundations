///https://www.hackingwithrust.net/2024/04/01/rustling-up-fun-a-simple-dive-into-thread-pool-patterns-with-rust/
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    queue: Arc<Mutex<Vec<WebRequest>>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        let queue = Arc::new(Mutex::new(Vec::<WebRequest>::new()));

        for i in 0..size {
            let queue = Arc::clone(&queue);
            workers.push(thread::spawn(move || loop {
                let task = queue.lock().unwrap().pop();
                match task {
                    Some(task) => (task.work)(&format!("worker {}", i)),
                    None => break,
                }
            }));
        }

        return ThreadPool { workers, queue };
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        let task = WebRequest::new(f);
        self.queue.lock().unwrap().push(task);
    }

    fn join(self) {
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}

struct WebRequest {
    work: Box<dyn FnOnce(&str) + Send + Sync>,
}

impl WebRequest {
    fn new<F>(f: F) -> Self
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        return WebRequest { work: Box::new(f) };
    }
}

fn main() {
    let pool = ThreadPool::new(6);
    for i in 0..6 {
        pool.execute(move |message| {
            println!("Task: {} prints  {}", i, message);
        });
    }
    pool.join();
}
