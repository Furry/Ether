use std::collections::HashMap;
use std::hash::Hash;

pub struct MultiKeyMap<K, V> {
    inner: HashMap<K, V>,
}

impl<K: Eq + Hash, V: Clone> MultiKeyMap<K, V> {
    pub fn new() -> Self {
        MultiKeyMap {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, keys: Vec<K>, value: V) -> Option<V> {
        let mut previous_value = None;
        for key in keys {
            previous_value = self.inner.insert(key, value.clone());
        }
        previous_value
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}