use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Type effectiveness factor
///
/// Remember: The factor is * 100 for easier integer representation (0.25x = 25, 2x = 200).
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TypeEffectivenessResponse(pub Option<i32>);

/// Type effectiveness factors mapped by type id
///
/// Remember: All factors are * 100 for easier integer representation (0.25x = 25, 2x = 200).
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AllTypeEffectivenessResponse(pub HashMap<i32, i32>);