use crate::entities::api::localized_values::LocalizedValuesMap;
use crate::entities::api::stat::Stat;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatsCSV {
    pub id: u32,
    pub damage_class_id: Option<u32>,
    pub identifier: String,
    pub is_battle_only: u32,
    pub game_index: Option<u32>,
}

impl CSVEntity for StatsCSV {
    fn file_name() -> &'static str {
        "stats"
    }
}

impl ApiCSVEntity for StatsCSV {
    type ApiType = Stat;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Stat {
            id: entry.id,
            identifier: entry.identifier,
            damage_class_id: entry.damage_class_id,
            is_battle_only: entry.is_battle_only == 1,
            game_index: entry.game_index,
            names: data.get(entry.id),
        })
    }
}