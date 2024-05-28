use std::any::Any;
use std::fmt::Debug;

fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i > j {
        return j;
    }

    return i;
}

#[derive(Debug, Clone)]
struct Cat(String);

#[derive(Debug, Clone)]
struct Dog(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} is fed", self.0);
    }
}
struct CatFeeder<'a> {
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

trait Animal: Debug {
    fn speak(&self);
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

fn speak_twice<A: Animal>(a: A) {
    a.speak();
    a.speak();
    println!("{:?}", a)
}

trait DowncastableAnimal {
    fn speak(&self);
    fn as_any(&self) -> &dyn Any;
}

struct Tortoise;

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn speak(&self) {
        println!("hiss")
    }
}

fn main() {
    /*1 lifetime*/
    println!("Hello, world!");
    let n = 32;
    let m = 42;
    borrow(&n, &m);

    /*2 lifetime */
    let mut cats = vec![Cat("Garfield".to_string()), Cat("Tom".to_string())];
    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder { cat });
    }

    feeders.iter_mut().for_each(|feeder| feeder.feed());

    /* 1 traits*/
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat("Garfield".to_string())),
        Box::new(Dog("Rex".to_string())),
    ];

    animals.iter().for_each(|a| a.speak());

    /* 2 traits */
    let animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];

    for a in animals {
        if let Some(c) = a.as_any().downcast_ref::<Cat>() {}
    }
}

fn the_problem() {
    struct Container(i32, i32);

    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains<i32, i32> for Container {
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

    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        return container.last() - container.first();
    }
}
