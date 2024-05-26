enum Color {
    Red,
    Green,
    Blue,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let num = 42;

    match num {
        1 | 2 => println!("one, two, three, or greater than three"),
        13..=42 => println!("between 13 and 42"),
        _ => println!("nothing special"),
    }

    let t = (1, 2.0, false);
    match t {
        // (x, f, b) => println!("{x}, {f}, {b}"),
        (1, ..) => println!("one first"),
        (0, ..) => println!("zero first"),
        (.., true) => println!("true last"),
        (x, y, z) => println!("{x}, {y}, {z}"),
    }

    let arr = [1, 2, 3, 4, 5];
    match arr {
        [1, .., 5] => println!("1, *, *, *, 5"),
        [.., 10] => println!("*, *, *, *, 10"),
        [3, second, tail_arr @ ..] => println!("3, {second}, {tail_arr:?}"),
        [first, middle_arr @ .., last] => println!("{first}, {middle_arr:?}, {last}"),
    }

    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
        Color::RGB(r, g, b) => println!("Red: {r}, Green: {g}, Blue: {b}!"),
        Color::HSV(h, s, v) => println!("Hue: {h}, Saturation: {s}, Value: {v}!"),
        Color::HSL(h, s, l) => println!("Hue: {h}, Saturation: {s}, Lightness: {l}!"),
        Color::CMY(c, m, y) => println!("Cyan: {c}, Magenta: {m}, Yellow: {y}!"),
        Color::CMYK(c, m, y, k) => println!("Cyan: {c}, Magenta: {m}, Yellow: {y}, Black: {k}!"),
    }

    let ptr = &4;
    match ptr {
        &val => {
            println!("Got a value via destructuring: {val}");
        }
    }

    match *ptr {
        val => {
            println!("Got a value via dereferencing: {val}");
        }
    }

    let ref new_ptr = 4;
    match new_ptr {
        &val => {
            println!("Got a value via destructuring: {val}");
        }
    }

    let val = 5;
    let mut val = 6;

    match val {
        ref r => println!("{:?}", r),
    }

    match val {
        ref mut r => {
            *r *= 10;
            println!("{:?}", r);
        }
    }

    println!("{val}");

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 1 };

    match foo {
        Foo {
            x: x @ (a, b),
            y: 1,
        } => println!("{x:?}, {a}, {b}"),
        Foo { x: (a, b), y: c } => println!("{a}, {b}, {c}"),
    }

    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }
    let temp = Temperature::Celsius(35);
    match temp {
        Temperature::Celsius(t) if t < 30 => println!("Celsius below 30: {t}"),
        Temperature::Celsius(t) => println!("Celsius above 30: {t}"),
        Temperature::Fahrenheit(t) if t < 40 => println!("Fahrenheit below 40: {t}"),
        Temperature::Fahrenheit(t) => println!("Fahrenheit above 40: {t}"),
    }

    let number = 4u8;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }

    // Bindiding:
    let f = || 42;

    match f() { 0 => println!("zero"),
        n @ 1..=10 => println!("1..=10: {n}"),
        n @ 11..=20 => println!("11..=20: {n}"),
        n => println!("other: {n}"),
    }

    let f = || Some(42);
    match f() {
        Some(n) if n == 42 => println!("{n}"), // Some(n @ 42) = println!("{n}"),
        Some(n @ 0..=41) => println!("{n}"),
        Some(n) => println!("{n}"),
        None => println!("none"),
    }
}
