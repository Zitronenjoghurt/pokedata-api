use crate::entities::api::localized_values::{LocalizedValues, VersionedLocalizedValues, VersionedLocalizedValuesMap};
use crate::entities::traits::has_localized_values::HasLocalizedValues;
use crate::entities::traits::has_version_id::HasVersionId;
use std::collections::HashMap;

pub trait IntoVersionedLocalizedValuesMap<T: HasLocalizedValues + HasVersionId> {
    fn into_versioned_localized_values_map(self) -> VersionedLocalizedValuesMap;
}

impl<T, C> IntoVersionedLocalizedValuesMap<T> for C
where
    T: HasLocalizedValues + HasVersionId,
    C: IntoIterator<Item=T>,
{
    fn into_versioned_localized_values_map(self) -> VersionedLocalizedValuesMap {
        let mut result: HashMap<u32, VersionedLocalizedValues> = HashMap::new();

        for item in self {
            result
                .entry(item.id())
                .or_insert_with(|| VersionedLocalizedValues(HashMap::new()))
                .0
                .entry(item.version_id())
                .or_insert_with(|| LocalizedValues(HashMap::new()))
                .0
                .insert(item.language_id(), item.name());
        }

        VersionedLocalizedValuesMap(result)
    }
}