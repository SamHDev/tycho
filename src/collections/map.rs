use std::collections::BTreeMap;

pub struct Map<K, V>(Vec<K>, Vec<V>);

impl<K, V> Map<K, V> {
    pub fn keys(&self) -> &Vec<K> { &self.0 }
    pub fn values(&self) -> &Vec<V> { &self.1 }
    pub fn set(&self) -> Vec<(&K, &V)> {
        self.0.iter().enumerate().map(|(i, k)| (k, &self.1[i])).collect()
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.0.push(k);
        self.1.push(v);
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.1.get(self.0.iter().position(k)?)
    }

    pub fn remove(&mut self, k: &K) ->
}