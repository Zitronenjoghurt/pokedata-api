use crate::api::localized_values::VersionGroupedLocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonMove {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
    pub type_id: u32,
    pub power: Option<u32>,
    pub pp: Option<u32>,
    pub accuracy: Option<u32>,
    pub priority: i32,
    pub target_id: u32,
    pub damage_class_id: u32,
    pub effect_id: Option<u32>,
    pub effect_chance: Option<u32>,
    pub contest_type_id: Option<u32>,
    pub contest_effect_id: Option<u32>,
    pub super_contest_effect_id: Option<u32>,
    pub flavor_texts: Option<VersionGroupedLocalizedValues>,
    pub flag_ids: Vec<u32>,
}

impl HasId for PokemonMove {
    fn id(&self) -> u32 {
        self.id
    }
}