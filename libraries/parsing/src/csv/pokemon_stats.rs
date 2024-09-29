use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::{HashMapGroupById, IdValuePairingMapped};
use pokedata_api_entities::api::pokemon_stats::PokemonStats;
use pokedata_api_entities::api::stat_value::StatValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonStatsCSV {
    pub pokemon_id: i32,
    pub stat_id: i32,
    pub base_stat: i32,
    pub effort: i32,
}

impl CSVEntity for PokemonStatsCSV {
    fn file_name() -> &'static str {
        "pokemon_stats"
    }
}

impl IdValuePairingMapped for PokemonStatsCSV {
    type Id = i32;
    type Key = i32;
    type Value = StatValue;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.pokemon_id, self.stat_id, StatValue { value: self.base_stat, effort: self.effort })
    }
}

impl PokemonStatsCSV {
    pub fn map(entries: Vec<PokemonStatsCSV>) -> HashMap<i32, PokemonStats> {
        let result = entries.group_by_id_mapped();
        result.into_iter()
            .map(|(pokemon_id, stats)| (pokemon_id, PokemonStats(stats)))
            .collect()
    }
}