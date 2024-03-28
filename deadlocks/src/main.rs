use std::sync::Mutex;
static MY_SHARED: Mutex<u32> = Mutex::new(0);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock = 42;
    panic!("poisoned");
}

fn main() {
    let h = std::thread::spawn(poisoner);
    println!("{}", MY_SHARED.lock().unwrap());
    println!("{:?}", h.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");

    let recovered_data = lock.unwrap_or_else(|poisoned| {
      println!("Mutex was poisoned");
      poisoned.into_inner()
    });
    
    println!("{}", recovered_data);
}
