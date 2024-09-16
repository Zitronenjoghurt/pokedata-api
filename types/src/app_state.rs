use crate::entities::api::ability::Ability;
use crate::entities::api::language::Language;
use crate::entities::api::species::Species;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    pub abilities: HashMap<u32, Ability>,
    pub languages: HashMap<u32, Language>,
    pub species: HashMap<u32, Species>,
}