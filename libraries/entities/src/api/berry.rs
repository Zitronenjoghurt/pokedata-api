use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Berry {
    pub id: i32,
    /// Berries are actually items. This is a reference to the item specific data for this berry.
    pub item_id: i32,
    /// The firmness of this berry, used in making Pokéblocks or Poffins.
    pub firmness_id: i32,
    /// The power of the move "Natural Gift" when used with this Berry.
    pub natural_gift_power: i32,
    /// The type inherited by "Natural Gift" when used with this Berry.
    pub natural_gift_type_id: i32,
    pub size_millimeter: i32,
    /// The maximum number of these berries that can grow on one tree in Generation IV.
    pub max_harvest: i32,
    /// Time it takes the tree to grow one stage, in hours. Berry trees go through four of these growth stages before they can be picked.
    pub growth_time: i32,
    /// The speed at which this Berry dries out the soil as it grows. A higher rate means the soil dries more quickly.
    pub soil_dryness: i32,
    /// The smoothness of this Berry, used in making Pokéblocks or Poffins.
    pub smoothness: i32,
    /// Flavor potency mapped by berry flavor id
    pub flavor_potencies: HashMap<i32, i32>,
}

impl HasId for Berry {
    fn id(&self) -> i32 {
        self.id
    }
}