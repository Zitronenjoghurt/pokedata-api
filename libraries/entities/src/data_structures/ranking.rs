use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranking<K, V>
where
    K: Copy + Eq + Hash,
    V: Copy + Ord,
{
    rankings: BTreeMap<usize, (K, V)>,
    key_rank_map: HashMap<K, usize>,
    descending: bool,
}

impl<K, V> Ranking<K, V>
where
    K: Copy + Eq + Hash,
    V: Ord + Copy,
{
    pub fn create<E, F>(entities: &HashMap<K, E>, value_extractor: F, descending: bool) -> Self
    where
        F: Fn(&E) -> V,
    {
        let mut values: Vec<(K, V)> = entities
            .iter()
            .map(|(&k, e)| (k, value_extractor(e)))
            .collect();

        if descending {
            values.sort_by(|a, b| b.1.cmp(&a.1));
        } else {
            values.sort_by(|a, b| a.1.cmp(&b.1));
        }

        let rankings: BTreeMap<usize, (K, V)> = values
            .into_iter()
            .enumerate()
            .map(|(index, (k, v))| (index + 1, (k, v)))
            .collect();

        let key_rank_map: HashMap<K, usize> = rankings.iter().map(|(rank, (k, _))| (*k, *rank)).collect();

        Self { rankings, key_rank_map, descending }
    }

    pub fn get_by_rank(&self, rank: usize) -> Option<(K, V)> {
        self.rankings.get(&rank).copied()
    }

    pub fn get_rank_by_key(&self, key: K) -> Option<usize> {
        self.key_rank_map.get(&key).copied()
    }

    pub fn get_all_rankings(&self) -> &BTreeMap<usize, (K, V)> {
        &self.rankings
    }

    pub fn is_descending(&self) -> bool {
        self.descending
    }
}