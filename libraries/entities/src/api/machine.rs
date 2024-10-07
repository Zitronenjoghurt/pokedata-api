use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Machine {
    pub id: i32,
    pub machine_number: i32,
    pub version_group_id: i32,
    pub item_id: i32,
    pub move_id: i32,
}

impl HasId for Machine {
    fn id(&self) -> i32 {
        self.id
    }
}