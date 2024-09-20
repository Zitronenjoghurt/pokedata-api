use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Ability {
    pub id: u32,
    pub is_main_series: bool,
}

impl HasId for Ability {
    fn id(&self) -> u32 {
        self.id
    }
}