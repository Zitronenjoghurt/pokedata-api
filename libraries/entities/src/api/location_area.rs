use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct LocationArea {
    pub id: i32,
    pub location_id: i32,
    pub game_index: i32,
    pub identifier: Option<String>,
    pub names: Option<LocalizedValues>,
    /// (encounter_method_id, rate) mapped by version_id. The rate is provided in percent.
    pub encounter_rates: HashMap<i32, (i32, i32)>,
    pub encounter_ids: Vec<i32>,
}

impl HasId for LocationArea {
    fn id(&self) -> i32 {
        self.id
    }
}