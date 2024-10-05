use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct BerryFlavor {
    pub id: i32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
    /// The contest type that correlates with this berry flavor.
    pub contest_type_id: i32,
}

impl HasId for BerryFlavor {
    fn id(&self) -> i32 {
        self.id
    }
}