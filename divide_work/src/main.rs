fn main() {
    const N_THREADS: usize = 4;
    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles = Vec::new();

    let chunks = to_add.chunks(N_THREADS);

    for c in chunks {
        let chunk = c.to_owned();
        thread_handles.push(std::thread::spawn(move || {
            chunk.iter().sum::<u32>()
        }));
    }

    let mut sum = 0;
    for h in thread_handles {
        sum += h.join().unwrap();
    }

    println!("sum = {}", sum);
}
