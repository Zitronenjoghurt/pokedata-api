use crate::api::localized_values::{LocalizedValues, VersionGroupedLocalizedValues};
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Item {
    pub id: i32,
    pub identifier: String,
    pub category_id: i32,
    pub cost: i32,
    pub flag_ids: Vec<i32>,
    pub game_indices: HashSet<i32>,
    pub generation_ids: HashSet<i32>,
    pub fling_power: Option<i32>,
    pub fling_effect_id: Option<i32>,
    pub names: Option<LocalizedValues>,
    pub short_effects: Option<LocalizedValues>,
    pub effects: Option<LocalizedValues>,
    pub flavor_texts: Option<VersionGroupedLocalizedValues>,
}

impl HasId for Item {
    fn id(&self) -> i32 {
        self.id
    }
}