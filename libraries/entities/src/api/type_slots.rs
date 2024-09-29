use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use std::collections::HashMap;
use utoipa::ToSchema;

/// Type id mapped by the slot number
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"1": 2, "2": 17}))]
pub struct TypeSlots(pub HashMap<u32, u32>);

/// Type slots mapped by the last generation id the pokemon had the given type combination
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"1": {"1": 2, "2": 14}, "...": {"...": "..."}}))]
pub struct TypeSlotsPast(pub HashMap<u32, TypeSlots>);