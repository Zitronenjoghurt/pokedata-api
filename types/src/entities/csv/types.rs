use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TypesCSV {
    pub id: Option<u32>,
    pub identifier: Option<String>,
    pub generation_id: Option<u32>,
    pub damage_class_id: Option<u32>,
    pub is_major_type: Option<u32>,
}

impl CSVEntity for TypesCSV {
    fn file_name() -> &'static str {
        "types"
    }

    fn base_download_url() -> &'static str {
        "https://raw.githubusercontent.com/Zitronenjoghurt/pokeapi-data-fix/master/data/v2/csv/"
    }
}