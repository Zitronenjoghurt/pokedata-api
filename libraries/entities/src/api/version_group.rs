use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct VersionGroup {
    pub id: u32,
    pub identifier: String,
    /// The generation this version group was introduced in.
    pub generation_id: u32,
    /// Order for sorting. Almost by date of release, except similar versions are grouped together.
    pub order: u32,
    /// The versions this version group owns.
    pub version_ids: Vec<u32>,
    /// A list of regions that can be visited in this version group.
    pub region_ids: Vec<u32>,
    /// A list of methods in which Pokémon can learn moves in this version group.
    pub move_method_ids: Vec<u32>,
}

impl HasId for VersionGroup {
    fn id(&self) -> u32 {
        self.id
    }
}