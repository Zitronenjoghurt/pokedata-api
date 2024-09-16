use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({"6": "german name", "9": "english name"}),
)]
pub struct LocalizedValues(pub HashMap<u32, String>);

#[derive(Clone, Debug)]
pub struct LocalizedValuesMap(pub HashMap<u32, LocalizedValues>);

impl LocalizedValuesMap {
    pub fn get(&self, entity_id: u32) -> Option<LocalizedValues> {
        self.0.get(&entity_id).cloned()
    }
}