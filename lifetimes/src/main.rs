fn main() {}

fn no_conflict_example_2() {
    let mut x = Box::new(42);
    let mut z = &x; // 'a

    for i in 0..100 {
        println!("{z}"); // 'a
        x = Box::new(i);
        z = &x; // 'a
    }

    println!("{z}"); // 'a
}

fn no_conflict_example_1() {
    let mut x = Box::new(42);
    let r = &x; // 'a
    if rand::random::<f32>() > 0.5 {
        *x *= 2;
    } else {
        println!("{r}"); // 'a
    }
}

use std::fmt::Display;
fn the_longest_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("ann: {ann}");
    return if x.len() > y.len() { x } else { y };
}
