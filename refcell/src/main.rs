use std::{cell::RefCell, sync::Arc};

struct Shareble {
    data: RefCell<String>,
}
impl Shareble {
    fn new(s: &str) -> Self {
        return Self {
            data: RefCell::new(s.to_string()),
        };
    }
}
fn move_data(data: Arc<Shareble>) {
    let mut data = data.data.borrow_mut();
    data.push_str(" World!");
}

fn main() {
    let shared = Arc::new(Shareble::new("Hello"));
    move_data(shared.clone());
    let data = shared.data.borrow();
    println!("{data}");
}
