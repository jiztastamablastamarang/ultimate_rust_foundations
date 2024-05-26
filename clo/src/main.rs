use std::iter::Iterator;

fn main() {
    let outer_var = 42;

    let clo_ann = |n: i32| -> i32 { n + outer_var };
    let clo_inf = |n| n + outer_var;

    println!("{} {}", clo_ann(1), clo_inf(1));

    let one = || 1;
    println!("{}", one());

    //##
    use std::mem;
    let color = String::from("green");
    let print = || println!("`color`: {}", color);

    print();
    let _color_moved = color;
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();
    inc();

    let _count_reborrowed = &mut count;

    let movable = Box::new(3);
    let consume = move || {
        println!("`movable`: {:?}", movable);
    };

    consume();

    // ##
    let haystack = [1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // ##

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        drop(farewell);
    };

    apply(diary);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double))
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    return f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    return f(3);
}

fn calling() {
    call_me(func);
}

fn call_me<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn func() {
    println!("func() called");
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    return move || println!("This is a: {text}");
}

fn create_fn_mut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    return move || println!("This is a: {text}");
}

fn create_fn_once() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    return move || println!("This is a: {text}");
}

fn check_create_funcs() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    fn_plain();
    fn_mut();
    fn_once();
}

fn check_iterator() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in {vec1:?}: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in {vec2:?}: {}", vec2.into_iter().any(|x| x == 2));

    println!("Find 2 in {vec1:?}: {:?}", vec1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in {vec2:?}: {:?}",
        vec2.into_iter().find(|&x| x == 2)
    );
}

fn hof() {
    fn apply(f: fn(i32) -> i32, y: i32) -> i32 {
        return f(y);
    }
    let square = |x| x * x;
    let cube = |x| x * x * x;
    apply(square, 2);
    apply(cube, 2);

    //
    fn is_odd(n: u32) -> bool {
        return n % 2 == 1;
    }
    let upper = 1000;

    let sum_of_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .sum();
}

fn foo() -> ! {
    panic!("This call never returns.")
}

fn sum_odd_numbers(upper: u32) -> u32 {
    let mut acc = 0;
    for n in 0..upper {
        let add: u32 = match n % 2 == 1 {
            true => n,
            false => continue,
        };
        acc += add;
    }

    return acc;
}
