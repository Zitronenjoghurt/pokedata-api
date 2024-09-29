use crate::traits::has_localized_values::HasLocalizedValues;
use pokedata_api_entities::api::localized_values::{LocalizedValues, LocalizedValuesMap};
use std::collections::HashMap;

pub trait IntoLocalizedValuesMap<T: HasLocalizedValues> {
    fn into_localized_values_map(self) -> LocalizedValuesMap;
}

impl<T, C> IntoLocalizedValuesMap<T> for C
where
    T: HasLocalizedValues,
    C: IntoIterator<Item=T>,
{
    fn into_localized_values_map(self) -> LocalizedValuesMap {
        let mut result: HashMap<u32, LocalizedValues> = HashMap::new();

        for item in self {
            result
                .entry(item.id())
                .or_insert_with(|| LocalizedValues(HashMap::new()))
                .0
                .insert(item.language_id(), item.name());
        }

        LocalizedValuesMap(result)
    }
}