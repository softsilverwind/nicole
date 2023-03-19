use std::{fmt::Debug, marker::PhantomData};

use crate::identifier::IdLike;

#[derive(Default, Debug, Clone)]
pub struct IdMap<K, V> {
    set: Vec<Option<V>>,
    phantom: PhantomData<K>
}

impl<K, V> IdMap<K, V>
    where K: IdLike,
{
    pub fn new() -> Self
    {
        Self {
            set: Vec::new(),
            phantom: PhantomData
        }
    }

    pub fn contains_key(&self, key: &K) -> bool
    {
        let pos: usize = (*key).into();
        pos < self.set.len() && self.set[pos].is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    {
        let pos: usize = (*key).into();
        let mut ret = None;

        if pos < self.set.len() {
            std::mem::swap(&mut ret, &mut self.set[pos]);
        }

        ret
    }

    pub fn insert(&mut self, key: K, value: V)
    {
        let pos: usize = key.into();

        if pos >= self.set.len() {
            self.set.resize_with(pos + 1, || None);
        }

        self.set[pos] = Some(value);
    }

    pub fn iter_clone<'a>(&'a self) -> impl Iterator<Item=(K, V)> + 'a
        where V: Clone
    {
        self.set
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(id, elem)| elem.map(|x| (id.into(), x)))
    }

    pub fn iter_copy(&self) -> impl Iterator<Item=(K, &V)>
        where V: Clone
    {
        self.set
            .iter()
            .enumerate()
            .filter_map(|(id, elem)| elem.as_ref().map(|x| (id.into(), x)))
    }

    pub fn max_key(&self) -> K
    {
        self.set
            .iter()
            .enumerate()
            .rfind(|(_, x)| x.is_some()).map(|(i, _)| i.into())
            .unwrap_or_else(|| K::null())
    }
}
