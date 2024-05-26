#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Tuple(i32, f32);

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
    Tuple(i32, f32),
}

fn inspect_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
        WebEvent::Tuple(l, r) => println!("Tuple({}, {})", l, r),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

enum HttpStatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let ok = HttpStatusCode::Ok;
    let num = ok as u32 + 1;

    let ops = Operations::Add;
    let _ops = VeryVerboseEnumOfThingsToDoWithNumbers::Add;

    let point1: Point = Point { x: 10.3, y: 0.4 };

    let point2 = Point { x: 0.3, ..point1 };

    let Point { x: left, y: right } = point1;

    println!("{:?}", point1);
    println!("{:?}", point2);
}

