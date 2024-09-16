use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GrowthRateProseCSV {
    pub growth_rate_id: u32,
    pub local_language_id: u32,
    pub name: String,
}

impl CSVEntity for GrowthRateProseCSV {
    fn file_name() -> &'static str {
        "growth_rate_prose"
    }
}

impl HasLocalizedValues for GrowthRateProseCSV {
    fn id(&self) -> u32 {
        self.growth_rate_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}