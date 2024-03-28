static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let h = std::thread::spawn(|| {
            for _ in 0..100 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("COUNTER: {}", unsafe { COUNTER });
}
