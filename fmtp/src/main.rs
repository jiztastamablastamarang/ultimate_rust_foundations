use std::io;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Deep(Structure);
fn main() {
    println!(
        "{subject}, {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("{struct:?}", struct = Deep(Structure(1)));
    println!("{pretty_person:#?}", pretty_person = Person { name: "Alice", age: 30 });
}
