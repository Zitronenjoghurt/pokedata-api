use crate::api::ability_changelog_entry::AbilityChangelogEntry;
use crate::api::localized_values::{LocalizedValues, VersionGroupedLocalizedValues};
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Ability {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub is_main_series: bool,
    pub names: Option<LocalizedValues>,
    pub effects: Option<LocalizedValues>,
    pub short_effects: Option<LocalizedValues>,
    pub flavor_texts: Option<VersionGroupedLocalizedValues>,
    /// Changelog entries mapped by the version group id they were introduced in
    pub changelogs: HashMap<i32, AbilityChangelogEntry>,
}

impl HasId for Ability {
    fn id(&self) -> i32 {
        self.id
    }
}