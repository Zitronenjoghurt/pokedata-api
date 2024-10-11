use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct EncounterSlot {
    pub id: i32,
    pub version_group_id: i32,
    pub encounter_method_id: i32,
    pub slot: Option<i32>,
    pub rarity: i32,
}

impl HasId for EncounterSlot {
    fn id(&self) -> i32 {
        self.id
    }
}