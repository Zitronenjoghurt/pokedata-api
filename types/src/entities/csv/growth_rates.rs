use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GrowthRatesCSV {
    pub id: Option<u32>,
    pub identifier: Option<String>,
    pub formula: Option<String>,
}

impl CSVEntity for GrowthRatesCSV {
    fn file_name() -> &'static str {
        "growth_rates"
    }
}