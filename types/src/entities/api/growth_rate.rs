use crate::entities::api::localized_values::LocalizedValues;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GrowthRate {
    pub id: u32,
    pub identifier: String,
    pub formula: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for GrowthRate {
    fn id(&self) -> u32 {
        self.id
    }
}