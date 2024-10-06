use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct EggGroup {
    pub id: i32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for EggGroup {
    fn id(&self) -> i32 {
        self.id
    }
}