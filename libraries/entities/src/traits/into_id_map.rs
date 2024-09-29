use crate::traits::has_id::HasId;
use std::collections::HashMap;

pub trait IntoIdMap<T: HasId> {
    fn into_id_map(self) -> HashMap<i32, T>;
}

impl<T, C> IntoIdMap<T> for C
where
    T: HasId,
    C: IntoIterator<Item=T>,
{
    fn into_id_map(self) -> HashMap<i32, T> {
        self.into_iter().map(|item| (item.id(), item)).collect()
    }
}