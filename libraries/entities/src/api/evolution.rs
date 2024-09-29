use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Evolution {
    pub id: i32,
    pub evolved_species_id: i32,
    pub evolution_trigger_id: i32,
    pub trigger_item_id: Option<i32>,
    pub minimum_level: Option<i32>,
    pub gender_id: Option<i32>,
    pub location_id: Option<i32>,
    pub held_item_id: Option<i32>,
    pub time_of_day: Option<String>,
    pub known_move_id: Option<i32>,
    pub known_move_type_id: Option<i32>,
    pub minimum_happiness: Option<i32>,
    pub minimum_beauty: Option<i32>,
    pub minimum_affection: Option<i32>,
    pub relative_physical_stats: Option<i32>,
    pub party_species_id: Option<i32>,
    pub party_type_id: Option<i32>,
    pub trade_species_id: Option<i32>,
    pub needs_overworld_rain: bool,
    pub turn_upside_down: bool,
}

impl HasId for Evolution {
    fn id(&self) -> i32 {
        self.id
    }
}