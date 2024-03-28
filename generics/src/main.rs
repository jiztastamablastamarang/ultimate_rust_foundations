use std::collections::HashMap;

struct Bucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> Bucket<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }
    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

struct BucketIter;

impl Iterator for BucketIter {
    type Item = ();
    fn next(& mut self) -> Option<Self::Item> {
        todo!();
    }
}


fn main() {
    let mut bucket = Bucket::new();
    bucket.insert("one", 1);
    bucket.insert("two", 2);
}
