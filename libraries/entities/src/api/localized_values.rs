use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use std::collections::HashMap;
use utoipa::ToSchema;

/// Strings mapped by language id
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({"6": "german name", "9": "english name", "...": "..."}),
)]
pub struct LocalizedValues(pub HashMap<u32, String>);

impl LocalizedValues {
    pub fn get(&self, language_id: u32) -> Option<String> {
        self.0.get(&language_id).cloned()
    }
}

#[derive(Clone, Debug, Default)]
pub struct LocalizedValuesMap(pub HashMap<u32, LocalizedValues>);

impl LocalizedValuesMap {
    pub fn get(&self, entity_id: u32) -> Option<LocalizedValues> {
        self.0.get(&entity_id).cloned()
    }
}

/// LocalizedValues mapped by their version id
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({"1": {"6": "german name", "9": "english name", "...": "..."}, "2": {"6": "german name", "9": "english name", "...": "..."}, "...": {"...": "..."}}),
)]
pub struct VersionedLocalizedValues(pub HashMap<u32, LocalizedValues>);

#[derive(Clone, Debug, Default)]
pub struct VersionedLocalizedValuesMap(pub HashMap<u32, VersionedLocalizedValues>);

impl VersionedLocalizedValuesMap {
    pub fn get(&self, entity_id: u32) -> Option<VersionedLocalizedValues> {
        self.0.get(&entity_id).cloned()
    }
}

/// LocalizedValues mapped by their version group id
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
#[schema(
    example = json!({"1": {"6": "german name", "9": "english name", "...": "...."}, "2": {"6": "german name", "9": "english name", "...": "..."}, "...": {"...": "..."}}),
)]
pub struct VersionGroupedLocalizedValues(pub HashMap<u32, LocalizedValues>);

#[derive(Clone, Debug, Default)]
pub struct VersionGroupedLocalizedValuesMap(pub HashMap<u32, VersionGroupedLocalizedValues>);

impl VersionGroupedLocalizedValuesMap {
    pub fn get(&self, entity_id: u32) -> Option<VersionGroupedLocalizedValues> {
        self.0.get(&entity_id).cloned()
    }
}