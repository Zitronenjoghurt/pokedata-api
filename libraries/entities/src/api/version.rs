use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Version {
    pub id: i32,
    /// The version group this version belongs to.
    pub version_group_id: i32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for Version {
    fn id(&self) -> i32 {
        self.id
    }
}