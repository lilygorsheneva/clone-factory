use std::{collections::HashMap, fmt::Display, hash::Hash};

struct StaticDataMap<K: Hash + Eq + Display, V: 'static> {
    map: HashMap<K, &'static V>
}

impl<K: Hash + Eq + Display, V> StaticDataMap<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            panic!("Duplicate entry {} in StaticDataMap", key)
        }
        let tmp = Box::new(value);
        self.map.insert(key, Box::leak(tmp));
    }

    fn get(&self, key:K) -> Option<&'static V> {
        let tmp = self.map.get(&key);
        match tmp {
            None => None,
            Some(foo) => Some(*foo)
        }
    }
}