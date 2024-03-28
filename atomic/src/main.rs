use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let h = std::thread::spawn(|| {
            for _ in 0..10 {
                COUNTER.fetch_add(1, Relaxed);
            }
        });

        handles.push(h);
    }

    handles.into_iter().map(|h| h.join().unwrap()).count();

    println!("COUNTER: {}", COUNTER.load(Relaxed));
}
