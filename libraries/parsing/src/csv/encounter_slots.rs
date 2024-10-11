use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::encounter_slot::EncounterSlot;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterSlotsCSV {
    pub id: i32,
    pub version_group_id: i32,
    pub encounter_method_id: i32,
    pub slot: Option<i32>,
    pub rarity: i32,
}

impl CSVEntity for EncounterSlotsCSV {
    fn file_name() -> &'static str {
        "encounter_slots"
    }
}

impl ApiCSVEntity for EncounterSlotsCSV {
    type ApiType = EncounterSlot;
    type ConversionData = ();

    fn convert(entry: Self, _data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(EncounterSlot {
            id: entry.id,
            version_group_id: entry.version_group_id,
            encounter_method_id: entry.encounter_method_id,
            slot: entry.slot,
            rarity: entry.rarity,
        })
    }
}