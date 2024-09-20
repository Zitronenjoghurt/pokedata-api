use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonTypeEfficacyEntry {
    pub damage_type_id: u32,
    pub target_type_id: u32,
    pub damage_factor: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PokemonTypeEfficacies(pub HashMap<u32, Vec<PokemonTypeEfficacyEntry>>);

impl PokemonTypeEfficacies {
    pub fn get_damage_factors_multi_attackers(&self, generation_id: u32, attacking_types: Vec<u32>, defending_types: Vec<u32>) -> HashMap<u32, u32> {
        attacking_types.into_iter()
            .filter_map(|attacking_type| {
                self.get_damage_factor_multi_defenders(generation_id, attacking_type, defending_types.clone())
                    .map(|factor| (attacking_type, factor))
            })
            .collect()
    }

    pub fn get_damage_factor_multi_defenders(&self, generation_id: u32, attacking_type: u32, defending_types: Vec<u32>) -> Option<u32> {
        if defending_types.is_empty() {
            return None;
        }

        let mut total_factor: u64 = 100;

        for defending_type in defending_types {
            if let Some(factor) = self.get_damage_factor(generation_id, attacking_type, defending_type) {
                total_factor = total_factor * u64::from(factor) / 100;
            }
        }

        Some((total_factor as u32).min(u32::MAX))
    }

    pub fn get_damage_factor(&self, generation_id: u32, attacking_type: u32, defending_type: u32) -> Option<u32> {
        self.0.get(&generation_id)
            .and_then(|entries| {
                entries.iter()
                    .find(|entry| entry.damage_type_id == attacking_type && entry.target_type_id == defending_type)
                    .map(|entry| entry.damage_factor)
            })
    }
}