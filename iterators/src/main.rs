struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        };
    }
}

fn main() {
    let numbers: Vec<u32> = Counter::new(5).collect();
    numbers.iter().for_each(|x| println!("{}", x));
}
