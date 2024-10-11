use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct EncounterConditionValue {
    pub id: i32,
    pub encounter_condition_id: i32,
    pub identifier: String,
    pub is_default: bool,
    pub names: Option<LocalizedValues>,
}

impl HasId for EncounterConditionValue {
    fn id(&self) -> i32 {
        self.id
    }
}