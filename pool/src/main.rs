mod pool;
use crate::pool::{Connection, Identifier, Pool};

fn main() {
    let mut pool = Pool::<Box<dyn Identifier>>::new();
    pool.add(Box::new(Connection::new(1)));
    pool.add(Box::new(Connection::new(2)));
    pool.add(Box::new(Connection::new(3)));

    let conn_1 = pool.get().unwrap();
    let conn_2 = pool.get().unwrap();
    let conn_3 = pool.get().unwrap();
    
    println!("Connection:{}", conn_1.get_id());
    println!("Connection:{}", conn_2.get_id());
    println!("Connection:{}", conn_3.get_id());
}
