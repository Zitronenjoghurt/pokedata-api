use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct DebugStats {
    pub tcg_cards_missing_dex_numbers: Vec<String>,
}