use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({"6": "german name", "9": "english name"}),
)]
pub struct LocalizedNames(pub HashMap<u32, String>);

#[derive(Clone, Debug)]
pub struct LocalizedNamesMap(pub HashMap<u32, LocalizedNames>);

impl LocalizedNamesMap {
    pub fn get(&self, entity_id: u32) -> Option<LocalizedNames> {
        self.0.get(&entity_id).cloned()
    }
}