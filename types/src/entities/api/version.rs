use crate::entities::api::localized_values::{LocalizedValues, LocalizedValuesMap};
use crate::entities::csv::versions::VersionsCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Version {
    pub id: u32,
    /// The version group this version belongs to.
    pub version_group_id: u32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for Version {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_versions(
    versions_csv: Vec<VersionsCSV>,
    names_map: LocalizedValuesMap,
) -> Vec<Version> {
    let mut versions = Vec::with_capacity(versions_csv.len());

    for entry in versions_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let version_group_id = match entry.version_group_id {
            Some(version_group_id) => version_group_id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let version = Version {
            id,
            version_group_id,
            identifier,
            names: names_map.get(id),
        };

        versions.push(version);
    }

    versions
}