use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Evolution {
    pub id: u32,
    pub evolved_species_id: u32,
    pub evolution_trigger_id: u32,
    pub trigger_item_id: Option<u32>,
    pub minimum_level: Option<u32>,
    pub gender_id: Option<u32>,
    pub location_id: Option<u32>,
    pub held_item_id: Option<u32>,
    pub time_of_day: Option<String>,
    pub known_move_id: Option<u32>,
    pub known_move_type_id: Option<u32>,
    pub minimum_happiness: Option<u32>,
    pub minimum_beauty: Option<u32>,
    pub minimum_affection: Option<u32>,
    pub relative_physical_stats: Option<i32>,
    pub party_species_id: Option<u32>,
    pub party_type_id: Option<u32>,
    pub trade_species_id: Option<u32>,
    pub needs_overworld_rain: bool,
    pub turn_upside_down: bool,
}

impl HasId for Evolution {
    fn id(&self) -> u32 {
        self.id
    }
}