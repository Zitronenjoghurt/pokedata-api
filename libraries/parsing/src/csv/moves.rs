use crate::csv::move_flag_map::MoveFlagMapCSV;
use crate::csv::move_flavor_text::MoveFlavorTextCSV;
use crate::csv::move_meta::MoveMetaCSV;
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing::GroupById;
use crate::traits::into_version_grouped_localized_values_map::IntoVersionGroupedLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::VersionGroupedLocalizedValuesMap;
use pokedata_api_entities::api::pokemon_move::PokemonMove;
use pokedata_api_entities::traits::into_id_map::IntoIdMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MovesCSV {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
    pub type_id: u32,
    pub power: Option<u32>,
    pub pp: Option<u32>,
    pub accuracy: Option<u32>,
    pub priority: i32,
    pub target_id: u32,
    pub damage_class_id: u32,
    pub effect_id: Option<u32>,
    pub effect_chance: Option<u32>,
    pub contest_type_id: Option<u32>,
    pub contest_effect_id: Option<u32>,
    pub super_contest_effect_id: Option<u32>,
}

impl CSVEntity for MovesCSV {
    fn file_name() -> &'static str {
        "moves"
    }
}

impl ApiCSVEntity for MovesCSV {
    type ApiType = PokemonMove;
    type ConversionData = PokemonMoveConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        let meta = data.move_meta_map.get(&entry.id).cloned().unwrap_or_default();

        Ok(PokemonMove {
            id: entry.id,
            identifier: entry.identifier,
            generation_id: entry.generation_id,
            type_id: entry.type_id,
            power: entry.power,
            pp: entry.pp,
            accuracy: entry.accuracy,
            priority: entry.priority,
            target_id: entry.target_id,
            damage_class_id: entry.damage_class_id,
            effect_id: entry.effect_id,
            effect_chance: entry.effect_chance,
            contest_type_id: entry.contest_type_id,
            contest_effect_id: entry.contest_effect_id,
            super_contest_effect_id: entry.super_contest_effect_id,
            flavor_texts: data.flavor_text_map.get(entry.id),
            flag_ids: data.flag_id_map.get(&entry.id).cloned().unwrap_or_default(),
            category_id: meta.meta_category_id,
            ailment_id: meta.meta_ailment_id,
            min_hits: meta.min_hits,
            max_hits: meta.max_hits,
            min_turns: meta.min_turns,
            max_turns: meta.max_turns,
            drain: meta.drain,
            healing: meta.healing,
            crit_rate: meta.crit_rate,
            ailment_chance: meta.ailment_chance,
            flinch_chance: meta.flinch_chance,
            stat_chance: meta.stat_chance,
        })
    }
}

#[derive(Debug)]
pub struct PokemonMoveConversionData {
    pub move_meta_map: HashMap<u32, MoveMetaCSV>,
    pub flag_id_map: HashMap<u32, Vec<u32>>,
    pub flavor_text_map: VersionGroupedLocalizedValuesMap,
}

impl PokemonMoveConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            move_meta_map: MoveMetaCSV::load(data_path).unwrap().into_id_map(),
            flag_id_map: MoveFlagMapCSV::load(data_path).unwrap().group_by_id(),
            flavor_text_map: MoveFlavorTextCSV::load(data_path).unwrap().into_version_grouped_localized_values_map(),
        }
    }
}