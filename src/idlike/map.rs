use std::{fmt::Debug, marker::PhantomData, ops::{Index, IndexMut}};

use crate::identifier::IdLike;

#[derive(Default, Debug, Clone)]
pub struct IdMap<K, V> {
    set: Vec<Option<V>>,
    phantom: PhantomData<K>
}

impl<K, V> IdMap<K, V>
    where K: IdLike
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

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = K> + 'a
    {
        self.set
            .iter()
            .enumerate()
            .filter_map(|(id, elem)| elem.as_ref().map(|_| id.into()))
    }

    pub fn iter(&self) -> impl Iterator<Item=(K, &V)>
        where V: Clone
    {
        self.set
            .iter()
            .enumerate()
            .filter_map(|(id, elem)| elem.as_ref().map(|x| (id.into(), x)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(K, &mut V)>
        where V: Clone
    {
        self.set
            .iter_mut()
            .enumerate()
            .filter_map(|(id, elem)| elem.as_mut().map(|x| (id.into(), x)))
    }

    pub fn into_iter(self) -> impl Iterator<Item=(K, V)>
        where V: Clone
    {
        self.set
            .into_iter()
            .enumerate()
            .filter_map(|(id, elem)| elem.map(|x| (id.into(), x)))
    }

    pub fn max_key(&self) -> K
    {
        self.set
            .iter()
            .enumerate()
            .rfind(|(_, x)| x.is_some()).map(|(i, _)| i.into())
            .unwrap_or_else(|| K::null())
    }

    pub fn get(&self, key: &K) -> Option<&V>
    {
        self.set.get((*key).into()).and_then(|x| x.as_ref())
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    {
        self.set.get_mut((*key).into()).and_then(|x| x.as_mut())
    }
}

impl<K, V> Index<&K> for IdMap<K, V>
    where K: IdLike
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output
    {
        self.get(key).expect("Trying to get nonexistent element!")
    }
}

impl<K, V> IndexMut<&K> for IdMap<K, V>
    where K: IdLike
{
    fn index_mut(&mut self, key: &K) -> &mut Self::Output
    {
        self.get_mut(key).expect("Trying to get nonexistent element!")
    }
}
