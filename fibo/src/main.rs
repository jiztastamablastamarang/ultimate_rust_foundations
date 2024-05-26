struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr = self.next;
        self.next += curr;

        return Some(curr);
    }
}

fn fibonacci() -> Fibonacci {
    return Fibonacci { curr: 0, next: 1 };
}

fn main() {
    println!("The first terms of natural sequence are: ");
    for n in  0..=10 {
        println!("{n}");
    }
    
    println!("The first terms of the Fibonacci sequence are: ");
    for n in fibonacci().take(10) {
        println!("{n}");
    }
}
