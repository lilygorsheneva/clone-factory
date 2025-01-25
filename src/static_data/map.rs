use std::{collections::HashMap, fmt::Display, hash::Hash};

pub struct DataMap<K: Hash + Eq + Display + Clone, V: 'static + Clone> {
    map: HashMap<K, &'static V>
}

impl<K: Hash + Eq + Display + Clone, V: Clone> DataMap<K, V> {
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, &V>{
        return self.map.iter()
    }

    pub fn from_map(input: &HashMap<K, V>) -> DataMap<K, V> {
        let iter = input.iter();
        let mut out = DataMap {
            map: HashMap::new()
        };
        for (k, v) in iter {
            out.insert(k, v)
        }
        out
    }
 

    pub fn insert(&mut self, key: &K, value: &V) {
        if self.map.contains_key(key) {
            panic!("Duplicate entry {} in DataMap", key)
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