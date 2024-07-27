//! This is a hacky drop-in replacement meant to
//! be close to a `Vec<T>` but let us look things up by `Id`s.
//! NOTE that we use a `BTreeMap` than a `HashMap` because the
//! performance of `HashMap`s are much worse in a WASM environment.

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type Id = uuid::Uuid;
pub trait HasId {
    fn id(&self) -> &Id;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Collection<T: HasId> {
    values: Vec<T>,
    lookup: BTreeMap<Id, usize>,
}

impl<'de, T: HasId + Deserialize<'de>> Deserialize<'de>
    for Collection<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Vec::<T>::deserialize(deserializer)?;
        Ok(Collection::from(values))
    }
}

impl<T: HasId + Serialize> Serialize for Collection<T> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.values.serialize(serializer)
    }
}

impl<T: HasId> Collection<T> {
    fn reindex(&mut self) {
        self.lookup.clear();
        for (i, value) in self.values.iter().enumerate() {
            self.lookup.insert(*value.id(), i);
        }
    }

    pub fn first(&self) -> &T {
        &self.values[0]
    }

    pub fn by_idx(&self, idx: usize) -> &T {
        &self.values[idx]
    }

    pub fn by_idx_mut(&mut self, idx: usize) -> &mut T {
        &mut self.values[idx]
    }

    pub fn try_get(&self, id: &Id) -> Option<&T> {
        self.lookup.get(id).map(|idx| &self.values[*idx])
    }

    pub fn try_get_mut(&mut self, id: &Id) -> Option<&mut T> {
        self.lookup.get(id).map(|idx| &mut self.values[*idx])
    }

    pub fn push_front(&mut self, value: T) {
        self.values.insert(0, value);
        self.reindex();
    }

    pub fn push(&mut self, value: T) {
        let id = *value.id();
        self.values.push(value);
        self.lookup.insert(id, self.values.len() - 1);
    }

    pub fn remove(&mut self, id: &Id) {
        self.lookup.get(id).map(|idx| {
            self.values.remove(*idx);
        });
        self.reindex();
    }
}
impl<T: HasId> From<Vec<T>> for Collection<T> {
    fn from(values: Vec<T>) -> Collection<T> {
        let mut lookup = BTreeMap::default();
        for (i, value) in values.iter().enumerate() {
            lookup.insert(*value.id(), i);
        }
        Collection { values, lookup }
    }
}
impl<T: HasId> FromIterator<T> for Collection<T> {
    fn from_iter<I: IntoIterator<Item = T>>(
        iter: I,
    ) -> Collection<T> {
        let values = iter.into_iter().collect::<Vec<_>>();
        Collection::from(values)
    }
}

impl<T: HasId> Default for Collection<T> {
    fn default() -> Self {
        Collection::from(Vec::default())
    }
}

impl<T: HasId> Deref for Collection<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
impl<T: HasId> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
impl<T: HasId> Index<&Id> for Collection<T> {
    type Output = T;
    fn index(&self, index: &Id) -> &Self::Output {
        self.try_get(index).unwrap()
    }
}
impl<T: HasId> IndexMut<&Id> for Collection<T> {
    fn index_mut(&mut self, index: &Id) -> &mut Self::Output {
        self.try_get_mut(index).unwrap()
    }
}
