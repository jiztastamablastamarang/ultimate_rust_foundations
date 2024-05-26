fn main() {
    let mut optional = Some(0);

    while let Some(n) = optional {
        if n > 9 {
            optional = None;
        } else {
            println!("{}", n);
            optional = Some(n + 1);
        }
    }
}
