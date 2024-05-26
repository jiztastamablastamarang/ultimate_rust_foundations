/// https://www.hackingwithrust.net/2023/10/15/an-object-pool-in-rust-two-implementations/
use std::sync::{Arc, Mutex};

pub trait Identifier {
    fn get_id(&self) -> i16;
}

pub struct Connection {
    id: i16,
}
impl Connection {
    pub fn new(id: i16) -> Self {
        return Self { id };
    }
}

impl Identifier for Connection {
    fn get_id(&self) -> i16 {
        return self.id;
    }
}

pub struct Pool<T> {
    connections: Arc<Mutex<Vec<T>>>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        return Self {
            connections: Arc::new(Mutex::new(Vec::new())),
        };
    }

    pub fn add(&mut self, conn: T) {
        self.connections.lock().unwrap().push(conn);
    }

    pub fn get(&mut self) -> Option<T> {
        return self.connections.lock().ok()?.pop();
    }
}
