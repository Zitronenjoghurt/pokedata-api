use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonTypeEfficacyEntry {
    pub damage_type_id: i32,
    pub target_type_id: i32,
    pub damage_factor: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PokemonTypeEfficacies(pub HashMap<i32, Vec<PokemonTypeEfficacyEntry>>);

impl PokemonTypeEfficacies {
    pub fn get_damage_factors_multi_attackers(&self, generation_id: i32, attacking_types: Vec<i32>, defending_types: Vec<i32>) -> HashMap<i32, i32> {
        attacking_types.into_iter()
            .filter_map(|attacking_type| {
                self.get_damage_factor_multi_defenders(generation_id, attacking_type, defending_types.clone())
                    .map(|factor| (attacking_type, factor))
            })
            .collect()
    }

    pub fn get_damage_factor_multi_defenders(&self, generation_id: i32, attacking_type: i32, defending_types: Vec<i32>) -> Option<i32> {
        if defending_types.is_empty() {
            return None;
        }

        let mut total_factor: i64 = 100;

        for defending_type in defending_types {
            if let Some(factor) = self.get_damage_factor(generation_id, attacking_type, defending_type) {
                total_factor = total_factor * i64::from(factor) / 100;
            }
        }

        Some((total_factor as i32).min(i32::MAX))
    }

    pub fn get_damage_factor(&self, generation_id: i32, attacking_type: i32, defending_type: i32) -> Option<i32> {
        self.0.get(&generation_id)
            .and_then(|entries| {
                entries.iter()
                    .find(|entry| entry.damage_type_id == attacking_type && entry.target_type_id == defending_type)
                    .map(|entry| entry.damage_factor)
            })
    }
}