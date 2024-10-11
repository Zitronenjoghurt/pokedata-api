use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Encounter {
    pub id: i32,
    pub version_id: i32,
    pub location_area_id: i32,
    pub encounter_slot_id: i32,
    pub pokemon_id: i32,
    pub min_level: i32,
    pub max_level: i32,
    pub encounter_condition_value_ids: Vec<i32>,
}

impl HasId for Encounter {
    fn id(&self) -> i32 {
        self.id
    }
}