use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Stat {
    pub id: u32,
    pub identifier: String,
    pub damage_class_id: Option<u32>,
    pub is_battle_only: bool,
    pub game_index: Option<u32>,
    pub names: Option<LocalizedValues>,
}

impl HasId for Stat {
    fn id(&self) -> u32 {
        self.id
    }
}