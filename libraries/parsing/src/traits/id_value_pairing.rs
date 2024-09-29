use std::collections::HashMap;
use std::hash::Hash;

pub trait IdValuePairing {
    type Id: Eq + Hash;
    type Value;

    fn into_pair(self) -> (Self::Id, Self::Value);
}

pub trait GroupById<T>
where
    T: IdValuePairing,
{
    fn group_by_id(self) -> HashMap<T::Id, Vec<T::Value>>;
}

impl<T, I> GroupById<T> for I
where
    T: IdValuePairing,
    I: IntoIterator<Item=T>,
{
    fn group_by_id(self) -> HashMap<T::Id, Vec<T::Value>> {
        self.into_iter().fold(HashMap::new(), |mut acc, item| {
            let (id, value) = item.into_pair();
            acc.entry(id).or_insert_with(Vec::new).push(value);
            acc
        })
    }
}