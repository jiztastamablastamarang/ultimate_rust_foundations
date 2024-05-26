use std::fmt::Display;

struct List(Vec<i32>);

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{i}:{v}")?;
        }

        return write!(f, "]");
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl std::fmt::Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        return write!(
            f,
            "{city}, {latitude_abs:.3}{latitude}, {longitude_abs:.3}{longitude}",
            city = self.name,
            latitude_abs = self.lat.abs(),
            latitude = lat_c,
            longitude_abs = self.lon.abs(),
            longitude = lon_c
        );
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "RGB({r}, {g}, {b}) 0x{r:02X}{g:02X}{b:02X})",
            r = self.red,
            g = self.green,
            b = self.blue
        )
    }
}
fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{v}",);

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{}", color);
    }
}
