use crate::entities::csv::type_efficacy::TypeEfficacyCSV;
use crate::entities::csv::type_efficacy_past::TypeEfficacyPastCSV;
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
pub struct PokemonTypeEfficacies(HashMap<u32, Vec<PokemonTypeEfficacyEntry>>);

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

pub fn build_efficacies_by_generation(
    type_efficacies_csv: Vec<TypeEfficacyCSV>,
    type_efficacy_past_csv: Vec<TypeEfficacyPastCSV>,
    latest_gen: u32,
) -> PokemonTypeEfficacies {
    let default_efficacies = build_default_efficacies(type_efficacies_csv);
    let mut efficacies_by_gen: HashMap<u32, Vec<PokemonTypeEfficacyEntry>> = HashMap::new();

    // Group past efficacies by generation
    let past_efficacies_by_gen: HashMap<u32, Vec<&TypeEfficacyPastCSV>> = type_efficacy_past_csv
        .iter()
        .fold(HashMap::new(), |mut acc, entry| {
            acc.entry(entry.generation_id).or_insert_with(Vec::new).push(entry);
            acc
        });

    // Populate the HashMap for each generation
    for gen in 1..=latest_gen {
        let mut gen_efficacies = default_efficacies.clone();

        // Apply all past changes up to and including this generation
        for past_gen in (1..=gen).rev() {
            if let Some(past_entries) = past_efficacies_by_gen.get(&past_gen) {
                for &past_entry in past_entries {
                    if let Some(entry) = gen_efficacies.iter_mut().find(|e|
                        e.damage_type_id == past_entry.damage_type_id &&
                            e.target_type_id == past_entry.target_type_id
                    ) {
                        entry.damage_factor = past_entry.damage_factor;
                    }
                }
            }
        }

        efficacies_by_gen.insert(gen, gen_efficacies);
    }

    PokemonTypeEfficacies(efficacies_by_gen)
}

fn build_default_efficacies(type_efficacies_csv: Vec<TypeEfficacyCSV>) -> Vec<PokemonTypeEfficacyEntry> {
    type_efficacies_csv
        .into_iter()
        .map(|efficacy| PokemonTypeEfficacyEntry {
            damage_type_id: efficacy.damage_type_id,
            target_type_id: efficacy.target_type_id,
            damage_factor: efficacy.damage_factor,
        })
        .collect()
}