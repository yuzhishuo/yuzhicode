use std::sync::Mutex;

use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

struct KvDb (Arc<Vec<RwLock<HashMap<String, Vec<u8>>>>>);

impl KvDb {

    pub fn new(len: usize) -> Self {

        let mut dbs = Vec::with_capacity(len);

        for _i in 0..len {
            dbs.push(RwLock::new(HashMap::new()));
        }

        Self(Arc::new(dbs))
    }
    pub fn get(&self, k: &str) -> Vec<u8> {
        let dbs = self.0.clone();
        let reader = dbs[(self.hash(k) % dbs.len()) as usize].read().unwrap();
        reader.get(k).unwrap().to_owned()
    }

    fn hash(&self, k: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        k.to_owned().hash(&mut hasher);
        hasher.finish() as usize
    }

    pub fn insert(&self, k: &str, v: Vec<u8>) {
        let dbs = self.0.clone();
        let mut writer = dbs[(self.hash(k) % dbs.len()) as usize].write().unwrap();
        writer.insert(k.into(), v);
    }
}


fn main() {
    let date = Mutex::new(0);
    let _d2 = date.lock();
    let _d3 = date.lock(); // dated lock
}