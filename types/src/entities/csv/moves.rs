use crate::entities::api::pokemon_move::PokemonMove;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
    type ConversionData = ();

    fn convert(entry: Self, _data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
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
        })
    }
}