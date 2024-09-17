use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TypeEffectivenessResponse(pub Option<u32>);

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AllTypeEffectivenessResponse(pub HashMap<u32, u32>);