use crate::csv_entity::CSVEntity;
use pokedata_api_entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveMetaCSV {
    pub move_id: i32,
    pub meta_category_id: Option<i32>,
    pub meta_ailment_id: Option<i32>,
    pub min_hits: Option<i32>,
    pub max_hits: Option<i32>,
    pub min_turns: Option<i32>,
    pub max_turns: Option<i32>,
    pub drain: Option<i32>,
    pub healing: Option<i32>,
    pub crit_rate: Option<i32>,
    pub ailment_chance: Option<i32>,
    pub flinch_chance: Option<i32>,
    pub stat_chance: Option<i32>,
}

impl HasId for MoveMetaCSV {
    fn id(&self) -> i32 {
        self.move_id
    }
}

impl CSVEntity for MoveMetaCSV {
    fn file_name() -> &'static str {
        "move_meta"
    }
}