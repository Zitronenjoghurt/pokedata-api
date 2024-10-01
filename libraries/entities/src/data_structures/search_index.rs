use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchIndex<S, T>
where
    S: Eq + Hash,
{
    index: HashMap<S, T>,
}

impl<S, T> Default for SearchIndex<S, T>
where
    S: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S, T> SearchIndex<S, T>
where
    S: Eq + Hash,
{
    pub fn new() -> Self {
        SearchIndex {
            index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: S, value: T) {
        self.index.insert(key, value);
    }

    pub fn get(&self, key: &S) -> Option<&T> {
        self.index.get(key)
    }

    pub fn remove(&mut self, key: &S) -> Option<T> {
        self.index.remove(key)
    }

    pub fn contains_key(&self, key: &S) -> bool {
        self.index.contains_key(key)
    }
}

impl<T> SearchIndex<String, T> {
    pub fn insert_case_insensitive(&mut self, key: String, value: T) {
        let lowercase_key = key.trim().to_lowercase();
        self.index.insert(lowercase_key, value);
    }

    pub fn get_case_insensitive(&self, key: &str) -> Option<&T> {
        self.index.get(&key.trim().to_lowercase())
    }
}