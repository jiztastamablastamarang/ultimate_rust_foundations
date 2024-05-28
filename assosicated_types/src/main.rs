trait Contains {
    // not aliases!
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

struct Container(i32, i32);

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        return (&self.0 == number_1) && (&self.1 == number_2);
    }

    fn first(&self) -> i32 {
        return self.0;
    }

    fn last(&self) -> i32 {
        return self.1;
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    return container.last() - container.first();
}

fn main() {
    println!("Hello, world!");
}
