fn main() {
    let mut v = vec!["Jane", "Joe", "John"];

    for name in v.iter() {
        match name {
            &"Jane" => println!("Hello, Jane!"),
            _ => println!("Hello, {}!", name),
        }
    }

    for name in v.iter_mut() {
        *name = match name {
            &mut "Jane" => "Jen",
            _ => name,
        }
    }

    for name in v.into_iter() {
        match name {
            "Jen" => println!("Hello, Jen!"),
            _ => println!("Hello, {}!", name),
        }
    }
}
