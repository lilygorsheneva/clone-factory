use std::{collections::HashMap, fmt::Display, hash::Hash};

pub struct StaticDataMap<K: Hash + Eq + Display + Clone, V: 'static + Clone> {
    map: HashMap<K, &'static V>
}

impl<K: Hash + Eq + Display + Clone, V: Clone> StaticDataMap<K, V> {
    pub fn new() -> StaticDataMap<K, V> {
        StaticDataMap {
            map: HashMap::new()
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, &V>{
        return self.map.iter()
    }

    pub fn from_map(input: &HashMap<K, V>) -> StaticDataMap<K, V> {
        let iter = input.iter();
        let mut out = StaticDataMap {
            map: HashMap::new()
        };
        for (k, v) in iter {
            out.insert(k, v)
        }
        out
    }
 

    pub fn insert(&mut self, key: &K, value: &V) {
        if self.map.contains_key(key) {
            panic!("Duplicate entry {} in StaticDataMap", key)
        }
        let tmp = Box::new(value.clone());
        self.map.insert(key.clone(), Box::leak(tmp));
    }

    pub fn get(&self, key: &K) -> Option<&'static V> {
        let tmp = self.map.get(key);
        match tmp {
            None => None,
            Some(foo) => Some(*foo)
        }
    }
}