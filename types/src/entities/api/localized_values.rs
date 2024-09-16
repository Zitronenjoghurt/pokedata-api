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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct VersionedLocalizedValues(pub HashMap<u32, LocalizedValues>);

#[derive(Clone, Debug)]
pub struct VersionedLocalizedValuesMap(pub HashMap<u32, VersionedLocalizedValues>);

impl VersionedLocalizedValuesMap {
    pub fn get(&self, entity_id: u32) -> Option<VersionedLocalizedValues> {
        self.0.get(&entity_id).cloned()
    }
}