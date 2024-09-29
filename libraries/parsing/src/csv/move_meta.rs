use crate::csv_entity::CSVEntity;
use pokedata_api_entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveMetaCSV {
    pub move_id: u32,
    pub meta_category_id: Option<u32>,
    pub meta_ailment_id: Option<u32>,
    pub min_hits: Option<u32>,
    pub max_hits: Option<u32>,
    pub min_turns: Option<u32>,
    pub max_turns: Option<u32>,
    pub drain: Option<i32>,
    pub healing: Option<u32>,
    pub crit_rate: Option<u32>,
    pub ailment_chance: Option<u32>,
    pub flinch_chance: Option<u32>,
    pub stat_chance: Option<u32>,
}

impl HasId for MoveMetaCSV {
    fn id(&self) -> u32 {
        self.move_id
    }
}

impl CSVEntity for MoveMetaCSV {
    fn file_name() -> &'static str {
        "move_meta"
    }
}