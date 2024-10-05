use crate::csv::berry_flavors::BerryFlavorPotenciesCSV;
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing_mapped::HashMapGroupById;
use pokedata_api_entities::api::berry::Berry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BerriesCSV {
    pub id: i32,
    pub item_id: i32,
    pub firmness_id: i32,
    pub natural_gift_power: i32,
    pub natural_gift_type_id: i32,
    pub size: i32,
    pub max_harvest: i32,
    pub growth_time: i32,
    pub soil_dryness: i32,
    pub smoothness: i32,
}

impl CSVEntity for BerriesCSV {
    fn file_name() -> &'static str {
        "berries"
    }
}

impl ApiCSVEntity for BerriesCSV {
    type ApiType = Berry;
    type ConversionData = BerryConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Berry {
            id: entry.id,
            item_id: entry.item_id,
            firmness_id: entry.firmness_id,
            natural_gift_power: entry.natural_gift_power,
            natural_gift_type_id: entry.natural_gift_type_id,
            size_millimeter: entry.size,
            max_harvest: entry.max_harvest,
            growth_time: entry.growth_time,
            soil_dryness: entry.soil_dryness,
            smoothness: entry.smoothness,
            flavor_potencies: data.berry_flavor_potency_map.get(&entry.id).cloned().unwrap_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct BerryConversionData {
    /// HashMap<berry_id, HashMap<flavor_id, potency>>
    pub berry_flavor_potency_map: HashMap<i32, HashMap<i32, i32>>,
}

impl BerryConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            berry_flavor_potency_map: BerryFlavorPotenciesCSV::load(data_path).unwrap().group_by_id_mapped(),
        }
    }
}