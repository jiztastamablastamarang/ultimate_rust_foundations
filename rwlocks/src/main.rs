use once_cell::sync::Lazy;
use std::sync::RwLock;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    return vec![
        "alice".to_string(),
        "bob".to_string(),
        "charlie".to_string(),
    ];
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_string();
}
fn main() {
    std::thread::spawn(|| loop {
        println!("Users");
        let users = USERS.read().unwrap();
        println!("{users:?}");
        std::thread::sleep(std::time::Duration::from_secs(3));
    });

    loop {
        let input = read_line();
        let mut lock = USERS.write().unwrap();
        lock.push(input);
    }
}
