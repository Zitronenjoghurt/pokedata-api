use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LanguagesCSV {
    pub id: Option<u32>,
    pub iso639: Option<String>,
    pub iso3166: Option<String>,
    pub identifier: Option<String>,
    pub official: Option<u32>,
    pub order: Option<u32>,
}

impl CSVEntity for LanguagesCSV {
    fn file_name() -> &'static str {
        "languages"
    }
}