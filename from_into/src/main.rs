use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct EventNumber(i32);

impl TryFrom<i32> for EventNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 != 0 {
            return Err(());
        }

        return Ok(EventNumber(value));
    }
}

struct Circle {
    radius: i32,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle of radius {}", self.radius);
    }
}

fn main() {
    assert_eq!(EventNumber::try_from(10), Ok(EventNumber(10)));
    assert_eq!(EventNumber::try_from(21), Err(()));
    assert_eq!(Circle { radius: 6 }.to_string(), "Circle of radius 6");

    // parsing a string into a numeric.
    assert_eq!("10".parse(), Ok(10));
    assert_eq!("10".parse::<i32>(), Ok(10));
}
