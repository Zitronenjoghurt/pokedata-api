use crate::csv::move_effect_changelog::MoveEffectChangelogCSV;
use crate::csv::move_effect_changelog_prose::MoveEffectChangelogProseCSV;
use crate::csv::move_effect_prose::{MoveEffectEffectsCSV, MoveEffectShortEffectsCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing_mapped::HashMapGroupById;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::pokemon_move_effect::PokemonMoveEffect;
use pokedata_api_entities::api::pokemon_move_effect_changelog_entry::PokemonMoveEffectChangelogEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveEffectsCSV {
    pub id: i32,
}

impl CSVEntity for MoveEffectsCSV {
    fn file_name() -> &'static str {
        "move_effects"
    }
}

impl ApiCSVEntity for MoveEffectsCSV {
    type ApiType = PokemonMoveEffect;
    type ConversionData = PokemonMoveEffectConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        let mut changelogs = data.effect_changelog_map.get(&entry.id).cloned().unwrap_or_default();
        for changelog_entry in changelogs.values_mut() {
            changelog_entry.effects = data.effect_changelog_prose_map.get(changelog_entry.id);
        }

        Ok(PokemonMoveEffect {
            id: entry.id,
            effects: data.effects_map.get(entry.id),
            short_effects: data.short_effects_map.get(entry.id),
            changelogs,
            move_ids: vec![],
        })
    }
}

#[derive(Debug)]
pub struct PokemonMoveEffectConversionData {
    pub effect_changelog_map: HashMap<i32, HashMap<i32, PokemonMoveEffectChangelogEntry>>,
    pub effect_changelog_prose_map: LocalizedValuesMap,
    pub effects_map: LocalizedValuesMap,
    pub short_effects_map: LocalizedValuesMap,
}

impl PokemonMoveEffectConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            effect_changelog_map: MoveEffectChangelogCSV::load(data_path).unwrap().group_by_id_mapped(),
            effect_changelog_prose_map: MoveEffectChangelogProseCSV::load(data_path).unwrap().into_localized_values_map(),
            effects_map: MoveEffectEffectsCSV::load(data_path).unwrap().into_localized_values_map(),
            short_effects_map: MoveEffectShortEffectsCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}