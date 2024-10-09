use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationAreaEncounterRatesCSV {
    pub location_area_id: i32,
    pub encounter_method_id: i32,
    pub version_id: i32,
    pub rate: i32,
}

impl CSVEntity for LocationAreaEncounterRatesCSV {
    fn file_name() -> &'static str {
        "location_area_encounter_rates"
    }
}

impl IdValuePairingMapped for LocationAreaEncounterRatesCSV {
    type Id = i32;
    type Key = i32;
    type Value = (i32, i32);

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.location_area_id, self.version_id, (self.encounter_method_id, self.rate))
    }
}