#[cfg(target_os = "windows")]
fn hello_os() {
    println!("Hello, Windows!");
}

#[cfg(target_os = "macos")]
fn hello_os() {
    println!("Hello, MacOS!");
}

fn main() {
    use std::collections::HashMap;

    let s = "Hello wonderful world";
    let mut map = HashMap::new();

    s.split_whitespace().into_iter().for_each(|word| {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    });
    println!("{:?}", map);
}
