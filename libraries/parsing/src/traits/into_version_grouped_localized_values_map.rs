use crate::traits::has_localized_values::HasLocalizedValues;
use crate::traits::has_version_group_id::HasVersionGroupId;
use pokedata_api_entities::api::localized_values::{LocalizedValues, VersionGroupedLocalizedValues, VersionGroupedLocalizedValuesMap};
use std::collections::HashMap;

pub trait IntoVersionGroupedLocalizedValuesMap<T: HasLocalizedValues + HasVersionGroupId> {
    fn into_version_grouped_localized_values_map(self) -> VersionGroupedLocalizedValuesMap;
}

impl<T, C> IntoVersionGroupedLocalizedValuesMap<T> for C
where
    T: HasLocalizedValues + HasVersionGroupId,
    C: IntoIterator<Item=T>,
{
    fn into_version_grouped_localized_values_map(self) -> VersionGroupedLocalizedValuesMap {
        let mut result: HashMap<i32, VersionGroupedLocalizedValues> = HashMap::new();

        for item in self {
            result
                .entry(item.id())
                .or_insert_with(|| VersionGroupedLocalizedValues(HashMap::new()))
                .0
                .entry(item.version_group_id())
                .or_insert_with(|| LocalizedValues(HashMap::new()))
                .0
                .insert(item.language_id(), item.name());
        }

        VersionGroupedLocalizedValuesMap(result)
    }
}