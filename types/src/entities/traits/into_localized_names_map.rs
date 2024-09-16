use crate::entities::api::localized_names::{LocalizedNames, LocalizedNamesMap};
use crate::entities::traits::has_localized_names::HasLocalizedNames;
use std::collections::HashMap;

pub trait IntoLocalizedNames<T: HasLocalizedNames> {
    fn into_localized_names_map(self) -> LocalizedNamesMap;
}

impl<T, C> IntoLocalizedNames<T> for C
where
    T: HasLocalizedNames,
    C: IntoIterator<Item=T>,
{
    fn into_localized_names_map(self) -> LocalizedNamesMap {
        let mut result: HashMap<u32, LocalizedNames> = HashMap::new();

        for item in self {
            result
                .entry(item.id())
                .or_insert_with(|| LocalizedNames(HashMap::new()))
                .0
                .insert(item.language_id(), item.name());
        }

        LocalizedNamesMap(result)
    }
}