use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Pokedex {
    pub id: i32,
    pub region_id: Option<i32>,
    pub identifier: String,
    pub is_main_series: bool,
    pub version_group_ids: Vec<i32>,
    pub names: Option<LocalizedValues>,
    pub descriptions: Option<LocalizedValues>,
}

impl HasId for Pokedex {
    fn id(&self) -> i32 {
        self.id
    }
}