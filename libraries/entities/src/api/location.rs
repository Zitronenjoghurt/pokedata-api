use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Location {
    pub id: i32,
    pub region_id: Option<i32>,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
    pub subtitles: Option<LocalizedValues>,
    pub game_indices: Vec<i32>,
    pub generation_ids: Vec<i32>,
}

impl HasId for Location {
    fn id(&self) -> i32 {
        self.id
    }
}