use once_cell::sync::Lazy;
use std::{collections::VecDeque, sync, sync::Mutex, thread, time::Duration};

static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

fn main() {
    let cpu_count = num_cpus::get();
    let mut threads = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);

    for cpu in 0..cpu_count {
        let (tx, rx) = sync::mpsc::channel::<()>();
        broadcast.push(tx);

        let thread = thread::spawn(move || {
            while rx.recv().is_ok() {
                println!("Hello from CPU {}", cpu);
                let mut lock = WORK_QUEUE.lock().unwrap();
                if let Some(work) = lock.pop_front() {
                    drop(lock);
                    println!("CPU {} is doing work: {}", cpu, work);
                    thread::sleep(Duration::from_millis(2_000));
                    println!("CPU {} finished work: {}", cpu, work);
                } else {
                    println!("CPU {} is idle", cpu);
                }
            }
        });

        threads.push(thread);
    }

    loop {
        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let len = lock.len();
            println!("Queue length: {}", len);

            if len < 5 {
                lock.push_back("Hello".to_string());
                true
            } else {
                false
            }
        };

        if sent {
            broadcast
                .iter()
                .for_each(|tx| tx.send(()).unwrap());
        }

        thread::sleep(Duration::from_millis(1_000));
    }
}
