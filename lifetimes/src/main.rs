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
