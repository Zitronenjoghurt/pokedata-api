use crate::csv::ability_changelog::AbilityChangelogCSV;
use crate::csv::ability_changelog_prose::AbilityChangelogProseCSV;
use crate::csv::ability_flavor_text::AbilityFlavorTextCSV;
use crate::csv::ability_names::AbilityNamesCSV;
use crate::csv::ability_prose::{AbilityEffectsCSV, AbilityShortEffectsCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing_mapped::HashMapGroupById;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use crate::traits::into_version_grouped_localized_values_map::IntoVersionGroupedLocalizedValuesMap;
use pokedata_api_entities::api::ability::Ability;
use pokedata_api_entities::api::ability_changelog_entry::AbilityChangelogEntry;
use pokedata_api_entities::api::localized_values::{LocalizedValuesMap, VersionGroupedLocalizedValuesMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilitiesCSV {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub is_main_series: i32,
}

impl CSVEntity for AbilitiesCSV {
    fn file_name() -> &'static str {
        "abilities"
    }
}

impl ApiCSVEntity for AbilitiesCSV {
    type ApiType = Ability;
    type ConversionData = AbilityConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        let mut changelogs = data.effect_changelog_map.get(&entry.id).cloned().unwrap_or_default();
        for changelog_entry in changelogs.values_mut() {
            changelog_entry.effects = data.effect_changelog_prose_map.get(changelog_entry.id);
        }

        Ok(Ability {
            id: entry.id,
            identifier: entry.identifier,
            generation_id: entry.generation_id,
            is_main_series: entry.is_main_series == 1,
            names: data.names_map.get(entry.id),
            effects: data.effects_map.get(entry.id),
            short_effects: data.short_effects_map.get(entry.id),
            flavor_texts: data.flavor_text_map.get(entry.id),
            changelogs,
        })
    }
}

#[derive(Debug)]
pub struct AbilityConversionData {
    pub flavor_text_map: VersionGroupedLocalizedValuesMap,
    pub names_map: LocalizedValuesMap,
    pub effects_map: LocalizedValuesMap,
    pub short_effects_map: LocalizedValuesMap,
    pub effect_changelog_map: HashMap<i32, HashMap<i32, AbilityChangelogEntry>>,
    pub effect_changelog_prose_map: LocalizedValuesMap,
}

impl AbilityConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            flavor_text_map: AbilityFlavorTextCSV::load(data_path).unwrap().into_version_grouped_localized_values_map(),
            names_map: AbilityNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            effects_map: AbilityEffectsCSV::load(data_path).unwrap().into_localized_values_map(),
            short_effects_map: AbilityShortEffectsCSV::load(data_path).unwrap().into_localized_values_map(),
            effect_changelog_map: AbilityChangelogCSV::load(data_path).unwrap().group_by_id_mapped(),
            effect_changelog_prose_map: AbilityChangelogProseCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}