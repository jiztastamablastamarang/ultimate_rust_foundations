async fn double(n: i32) -> i32 {
    return n * 2;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_double() {
        assert_eq!(double(2).await, 4);
    }
}
