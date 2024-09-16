use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedNames(pub HashMap<u32, String>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedNamesMap(pub HashMap<u32, LocalizedNames>);

impl LocalizedNamesMap {
    pub fn get(&self, entity_id: u32) -> Option<LocalizedNames> {
        self.0.get(&entity_id).cloned()
    }
}