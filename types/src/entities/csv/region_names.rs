use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegionNamesCSV {
    pub region_id: u32,
    pub local_language_id: u32,
    pub name: String,
}

impl CSVEntity for RegionNamesCSV {
    fn file_name() -> &'static str {
        "region_names"
    }
}

impl HasLocalizedValues for RegionNamesCSV {
    fn id(&self) -> u32 {
        self.region_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}