fn main() {
    let mut x;
    x = 42; 
    let mut y = &x;
    let z  = &mut y;
    *z = &42;
}
