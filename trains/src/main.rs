fn main() {
    assert_eq!(make_adder_function(3)(1), 3 + 1);
}

fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    return src
        .lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect();
}

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> std::iter::Cycle<std::iter::Chain<std::vec::IntoIter<i32>, std::vec::IntoIter<i32>>> {
    return v.into_iter().chain(u.into_iter()).cycle();
}

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl std::iter::Iterator<Item = i32> {
    return v.into_iter().chain(u.into_iter()).cycle();
}

fn make_adder_function(x: i32) -> impl Fn(i32) -> i32 {
    return move |y| x + y;
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    return numbers.iter().filter(|x| x > &&0).map(|x| x * 2);
}
