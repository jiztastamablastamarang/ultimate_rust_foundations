fn main() {
    let input = read_line();
    println!("{input}");
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");

    return buf.trim().to_string();
}

fn greet_borrow_mut(name: &mut String) {
    *name = format!("Hello, {}!", name);
}

fn greet_borrow(name: &String) {
    println!("Hello, {name}!");
}

fn greet(name: String) {
    println!("Hello, {name}!");
}

fn double(n: i32) -> i32 {
    return 2 * n;
}


