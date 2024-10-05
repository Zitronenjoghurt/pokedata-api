use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BerryFlavorPotenciesCSV {
    pub berry_id: i32,
    /// Actually referring to the flavor in the contest_type_names table
    pub contest_type_id: i32,
    /// This is the potency
    pub flavor: i32,
}

impl CSVEntity for BerryFlavorPotenciesCSV {
    fn file_name() -> &'static str {
        "berry_flavors"
    }
}

impl IdValuePairingMapped for BerryFlavorPotenciesCSV {
    type Id = i32;
    type Key = i32;
    type Value = i32;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.berry_id, self.contest_type_id, self.flavor)
    }
}