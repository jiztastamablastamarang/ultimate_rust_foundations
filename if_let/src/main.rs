fn main() {
    let num = Some(42);
    let letter: Option<char> = None;

    if let Some(n) = num {
        println!("{n}");
    } else {
        println!("Nothing");
    }

    if let Some(c) = letter {
        println!("{c}");
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Qux(val @ 100) = b {
        println!("b is 100 and val is {val}");
    } else if let Foo::Qux(n) = b {
        println!("b is not 100:  {n}");
    }
}

fn define_first_pair(s: &str) -> (u64, &str) {
    let mut it = s.splitn(2, ',');
    
    let (Some(first), Some(second)) = (it.next(), it.next()) else {
        panic!("invalid input: no comma found");
    };

    let Ok(first) = first.parse::<u64>() else {
        panic!("invalid input: first half not a number")
    };

    return (first, second);
}
