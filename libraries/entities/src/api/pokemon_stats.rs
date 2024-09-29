use crate::api::stat_value::StatValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Pokemon stat values mapped by stat id
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonStats(pub HashMap<u32, StatValue>);