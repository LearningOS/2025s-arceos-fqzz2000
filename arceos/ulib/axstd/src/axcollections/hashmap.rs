use alloc::vec::Vec;
use alloc::vec;
use core::hash::{Hash, Hasher};
pub struct HashMap<K, V> {
    table: Vec<Vec<(K, V)>>,
    size: usize,
}

struct DefaultHasher {
    state: usize,
}

impl DefaultHasher {
    fn new() -> Self {
        DefaultHasher { state: 42 }
    }
}

impl Hasher for DefaultHasher {

fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state = self.state.wrapping_mul(33).wrapping_add(*byte as usize);
        }
    }
fn finish(&self) -> u64 {
        self.state as u64
    }
}

impl<K:Hash+Eq + Clone , V:Clone> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap { table: vec![Vec::new(); 16], size: 0 }
    }

    fn hash(&self, msg: &K) -> usize{
        let mut hasher = DefaultHasher::new();
        msg.hash(&mut hasher);
        hasher.finish() as usize
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hash = self.hash(&key);
        let idx = hash % self.table.len();
        self.table[idx].push((key, value));
        self.size += 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let idx = hash % self.table.len();
        self.table[idx].iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            map: self,
            bucket: 0,
            index: 0,
        }
    }
}

pub struct Iter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    index: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket < self.map.table.len() {
            let bucket = &self.map.table[self.bucket];
            if self.index < bucket.len() {
                let result = Some((&bucket[self.index].0, &bucket[self.index].1));
                self.index += 1;
                return result;
            }
            self.bucket += 1;
            self.index = 0;
        }
        None
    }
}




