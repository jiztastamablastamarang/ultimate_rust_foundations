use std::thread;

fn main() {
    const N_TREADS: usize = 8;
    let to_add = (0..5000).collect::<Vec<u32>>();
    let chunks = to_add.chunks(N_TREADS);

    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();
        for c in chunks {
            let h = s.spawn(move || c.iter().sum::<u32>());
            thread_handles.push(h);
        }

        thread_handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<u32>()
    });

    println!("sum = {}", sum);
}
