use crate::entities::api::pokemon_stats::PokemonStats;
use crate::entities::api::stat_value::StatValue;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::id_value_pairing_mapped::{HashMapGroupById, IdValuePairingMapped};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonStatsCSV {
    pub pokemon_id: u32,
    pub stat_id: u32,
    pub base_stat: u32,
    pub effort: u32,
}

impl CSVEntity for PokemonStatsCSV {
    fn file_name() -> &'static str {
        "pokemon_stats"
    }
}

impl IdValuePairingMapped for PokemonStatsCSV {
    type Id = u32;
    type Key = u32;
    type Value = StatValue;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.pokemon_id, self.stat_id, StatValue { value: self.base_stat, effort: self.effort })
    }
}

impl PokemonStatsCSV {
    pub fn map(entries: Vec<PokemonStatsCSV>) -> HashMap<u32, PokemonStats> {
        let result = entries.group_by_id_mapped();
        result.into_iter()
            .map(|(pokemon_id, stats)| (pokemon_id, PokemonStats(stats)))
            .collect()
    }
}