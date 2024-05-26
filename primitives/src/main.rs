use std::mem;

fn main() {
    let num = "74".parse::<u32>().unwrap();
    let b =b'A' ;
    let x = 74.0;
    let heart_eyed_cat = '😻';
    let arr= [3;5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let raw_bytes = b"This is a byte string";

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys = &xs[..3];

    let (x, y) = analyze_slice(ys);

    let empty_arr: [u32; 0] = [];
    assert_eq!(&empty_arr, &[]);

    println!("{:?}", months);
    println!("{}", mem::size_of_val(&xs));
    println!("{x}, {y}");
    println!("{raw_bytes:?}");
    println!("{num}");
    println!("{arr:?}");
    println!("{b}");
    println!("{result}", result=32%3);
    println!("{heart_eyed_cat}")
}

fn analyze_slice(slice: &[i32]) -> (i32, i32) {
    let mut min = slice[0];
    let mut max = slice[0];
    for &v in slice {
        if v < min {
            min = v;
        } else if v > max {
            max = v;
        }
    }
    (min, max)
}