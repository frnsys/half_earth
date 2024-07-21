//! This is a hacky drop-in replacement meant to
//! be close to a `Vec<T>` but let us look things up by `Id`s.
//! Ideally we'd just use a `HashMap<Id, T>` but that change
//! may break a lot of things so I'd rather not.

use std::ops::{Deref, DerefMut, Index, IndexMut};

use serde::{Deserialize, Serialize};

pub type Id = uuid::Uuid;
pub trait HasId {
    fn id(&self) -> &Id;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Collection<T: HasId>(Vec<T>);
impl<T: HasId> Collection<T> {
    pub fn by_idx(&self, idx: usize) -> &T {
        &self.0[idx]
    }

    pub fn by_idx_mut(&mut self, idx: usize) -> &mut T {
        &mut self.0[idx]
    }

    pub fn try_get(&self, id: &Id) -> Option<&T> {
        self.0.iter().find(|item| item.id() == id)
    }
}
impl<T: HasId> Deref for Collection<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: HasId> Default for Collection<T> {
    fn default() -> Self {
        Collection(Vec::default())
    }
}
impl<T: HasId> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        self.0
            .iter_mut()
            .find(|item| item.id() == index)
            .unwrap()
    }
}
