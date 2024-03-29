use std::{
    iter::FromIterator,
    fmt::Debug
};

use crate::identifier::Identifier;

#[derive(Default, Clone)]
pub struct DenseSet<T> {
    elements: Vec<T>,
    indices: Vec<usize>
}

const INVALID: usize = usize::MAX;

impl<T> Debug for DenseSet<T>
    where
        T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("DenseSet").field("elements", &self.elements).finish()
    }
}

impl<T> DenseSet<T>
    where T: Identifier
{
    pub fn new() -> Self
    {
        Self {
            elements: Vec::new(),
            indices: Vec::new()
        }
    }

    pub fn contains(&self, value: &T) -> bool
    {
        let pos: usize = value.id().into();

        pos < self.indices.len() && self.indices[pos] != INVALID
    }

    fn remove_at(&mut self, elem_index: usize) -> Option<T>
    {
        if elem_index == INVALID {
            None
        }
        else if elem_index == self.elements.len() - 1 {
            self.elements.pop()
        }
        else {
            let ret = self.elements.swap_remove(elem_index);
            self.indices[self.elements[elem_index].id().into()] = elem_index;
            Some(ret)
        }
    }

    pub fn remove(&mut self, value: &T) -> Option<T>
    {
        let pos: usize = value.id().into();

        if pos < self.indices.len() {
            let ret = self.remove_at(self.indices[pos]);
            self.indices[pos] = INVALID;
            ret
        }
        else {
            None
        }
    }

    pub fn insert(&mut self, value: T) -> Option<T>
    {
        let pos: usize = value.id().into();

        if pos >= self.indices.len() {
            self.indices.resize(pos + 1, INVALID);
        }

        let ret = self.remove_at(self.indices[pos]);
        self.elements.push(value);
        self.indices[pos] = self.elements.len() - 1;

        ret
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a
    {
        self.elements.iter()
    }
}

impl<T> FromIterator<T> for DenseSet<T>
    where T: Identifier,
{
    fn from_iter<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = T>,
    {
        let mut ret = DenseSet::new();
        for item in iter.into_iter() {
            ret.insert(item);
        }

        ret
    }
}

impl<T> Extend<T> for DenseSet<T>
    where T: Identifier
{
    fn extend<IT>(&mut self, iter: IT)
    where
        IT: IntoIterator<Item = T>,
    {
        for x in iter {
            self.insert(x);
        }
    }
}

impl<T> IntoIterator for DenseSet<T>
    where T: Identifier
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {   
        self.elements.into_iter()
    }
}
