use std::{
    marker::PhantomData,
    ops::{Index, IndexMut, Deref, DerefMut}
};

use crate::IdLike;

#[derive(Clone, Debug, Default)]
pub struct TypedVec<K, V>
{
    inner: Vec<V>,
    phantom: PhantomData<K>
}

impl<K, V> Index<K> for TypedVec<K, V>
    where K: IdLike
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output { self.get(index).unwrap() }
}

impl<K, V> IndexMut<K> for TypedVec<K, V>
    where K: IdLike
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output { self.get_mut(index).unwrap() }
}

impl <K, V> Deref for TypedVec<K, V>
{
    type Target = Vec<V>;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl <K, V> DerefMut for TypedVec<K, V>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<K, V> TypedVec<K, V>
    where K: IdLike
{
    pub fn new() -> Self
    {
        Self { inner: Vec::new(), phantom: PhantomData }
    }

    pub fn get(&self, index: K) -> Option<&V>
    {
        self.inner.get(index.into())
    }

    pub fn get_mut(&mut self, index: K) -> Option<&mut V>
    {
        self.inner.get_mut(index.into())
    }

    pub fn contains(&self, index: K) -> bool
    {
        index.into() < self.inner.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=(K, &V)>
    {
        self.inner.iter().enumerate().map(|(i, x)| (i.into(), x))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(K, &mut V)>
    {
        self.inner.iter_mut().enumerate().map(|(i, x)| (i.into(), x))
    }

    pub fn into_iter(self) -> impl Iterator<Item=(K, V)>
    {
        self.inner.into_iter().enumerate().map(|(i, x)| (i.into(), x))
    }
}

impl<K, V> TypedVec<K, V>
    where K: IdLike, V: Default
{
    pub fn insert(&mut self, index: K, value: V)
    {
        let uindex = index.into();
        if uindex >= self.inner.len() {
            self.inner.resize_with(uindex + 1, || Default::default())
        }
        self.inner[uindex] = value;
    }
}
