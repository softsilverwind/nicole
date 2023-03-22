use std::{
    iter::FromIterator,
    fmt::Debug
};

use crate::identifier::IdLike;

#[derive(Default, Debug, Clone)]
pub struct IdSet<T> {
    set: Vec<bool>,
    phantom: std::marker::PhantomData<T>
}

impl<T> IdSet<T>
    where T: IdLike,
{
    pub fn new() -> Self
    {
        Self {
            set: Vec::new(),
            phantom: Default::default()
        }
    }

    pub fn contains(&self, value: &T) -> bool
    {
        let pos: usize = (*value).into();
        pos < self.set.len() && self.set[pos]
    }

    pub fn remove(&mut self, value: &T)
    {
        let pos: usize = (*value).into();

        if pos < self.set.len() {
            self.set[pos] = false;
        }
    }

    pub fn insert(&mut self, value: T)
    {
        let pos: usize = value.into();

        if pos >= self.set.len() {
            self.set.resize(pos + 1, false);
        }

        self.set[pos] = true;
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=T> + 'a
    {
        self.set
            .iter()
            .enumerate()
            .filter_map(|(id, &exists)| if exists { Some(T::from(id)) } else { None } )
    }

    pub fn max(&self) -> T
    {
        self.set
            .iter()
            .enumerate()
            .rfind(|(_, x)| **x).map(|(i, _)| i.into())
            .unwrap_or_else(|| T::null())
    }
}

impl<T> FromIterator<T> for IdSet<T>
    where T: IdLike,
{
    fn from_iter<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = T>,
    {
        let mut ret = IdSet::new();
        for item in iter.into_iter() {
            ret.insert(item);
        }

        ret
    }
}

impl<T> Extend<T> for IdSet<T>
    where T: IdLike
{
    fn extend<IT>(&mut self, iter: IT)
    where
        IT: IntoIterator<Item = T>,
    {
        for x in iter {
            self.insert(x)
        }
    }
}

impl<T> IntoIterator for IdSet<T>
    where T: IdLike
{
    type Item = T;
    // TODO check if impl Iterator<Item=T> applies after https://github.com/rust-lang/rust/issues/63063
    // has been merged
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {   
        self.set
            .into_iter()
            .enumerate()
            .filter_map(|(id, exists)| if exists { Some(T::from(id)) } else { None } )
            .collect::<Vec<_>>()
            .into_iter()
    }
}
