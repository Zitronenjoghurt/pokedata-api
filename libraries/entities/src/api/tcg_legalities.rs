use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgLegalities {
    pub standard: Option<String>,
    pub expanded: Option<String>,
    pub unlimited: String,
}