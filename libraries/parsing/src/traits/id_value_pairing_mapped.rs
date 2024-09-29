use std::collections::HashMap;
use std::hash::Hash;

pub trait IdValuePairingMapped {
    type Id: Eq + Hash;
    type Key: Eq + Hash;
    type Value;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value);
}

pub trait HashMapGroupById<T>
where
    T: IdValuePairingMapped,
{
    fn group_by_id_mapped(self) -> HashMap<T::Id, HashMap<T::Key, T::Value>>;
}

impl<T, I> HashMapGroupById<T> for I
where
    T: IdValuePairingMapped,
    I: IntoIterator<Item=T>,
{
    fn group_by_id_mapped(self) -> HashMap<T::Id, HashMap<T::Key, T::Value>> {
        self.into_iter().fold(HashMap::new(), |mut acc, item| {
            let (id, key, value) = item.into_triple();
            acc.entry(id)
                .or_insert_with(HashMap::new)
                .insert(key, value);
            acc
        })
    }
}