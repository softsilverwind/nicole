use std::{ops::{Index, IndexMut}, fmt::Debug};

use crate::IdLike;

#[derive(Default, Clone)]
pub struct DenseHashMap<K, V> {
    elements: Vec<(K, V)>,
    indices: Vec<usize>
}

const INVALID: usize = usize::MAX;

impl<K, V> Debug for DenseHashMap<K, V>
    where
        K: Debug,
        V: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("DenseHashMap").field("elements", &self.elements).finish()
    }
}

impl<K, V> DenseHashMap<K, V>
    where K: IdLike
{
    pub fn new() -> Self
    {
        Self {
            elements: Vec::new(),
            indices: Vec::new()
        }
    }

    pub fn contains_key(&self, key: &K) -> bool
    {
        let pos: usize = (*key).into();

        pos < self.indices.len() && self.indices[pos] != INVALID
    }

    fn remove_at(&mut self, elem_index: usize) -> Option<(K, V)>
    {
        if elem_index == INVALID {
            None
        }
        else if elem_index == self.elements.len() - 1 {
            self.elements.pop()
        }
        else {
            let ret = self.elements.swap_remove(elem_index);
            self.indices[self.elements[elem_index].0.into()] = elem_index;
            Some(ret)
        }
    }

    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)>
    {
        let pos: usize = (*key).into();

        if pos < self.indices.len() {
            let ret = self.remove_at(self.indices[pos]);
            self.indices[pos] = INVALID;
            ret
        }
        else {
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    {
        self.remove_entry(key).map(|x| x.1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        let pos: usize = key.into();

        if pos >= self.indices.len() {
            self.indices.resize(pos + 1, INVALID);
        }

        let ret = self.remove_at(self.indices[pos]);
        self.elements.push((key, value));
        self.indices[pos] = self.elements.len() - 1;

        ret.map(|x| x.1)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &(K, V)> + 'a
    {
        self.elements.iter()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = &K> + 'a
    {
        self.elements.iter().map(|x| &x.0)
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &V> + 'a
    {
        self.elements.iter().map(|x| &x.1)
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut (K, V)> + 'a
    {
        self.elements.iter_mut()
    }

    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, K, V>
    {
        Entry { parent: self, key }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    {
        if self.contains_key(key) {
            Some(&self.elements[self.indices[(*key).into()]].1)
        }
        else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    {
        if self.contains_key(key) {
            Some(&mut self.elements[self.indices[(*key).into()]].1)
        }
        else {
            None
        }
    }
}

impl<K, V> Index<&K> for DenseHashMap<K, V>
    where K: IdLike
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output
    {
        self.get(key).expect("Trying to get nonexistent element!")
    }
}

impl<K, V> IndexMut<&K> for DenseHashMap<K, V>
    where K: IdLike
{
    fn index_mut(&mut self, key: &K) -> &mut Self::Output
    {
        self.get_mut(key).expect("Trying to get nonexistent element!")
    }
}

pub struct Entry<'a, K, V>
{
    parent: &'a mut DenseHashMap<K, V>,
    key: K
}

impl<'a, K, V> Entry<'a, K, V>
    where
        K: IdLike + 'a,
        V: 'a
{
    pub fn or_insert(self, value: V) -> &'a mut V
    {
        if !self.parent.contains_key(&self.key) {
            self.parent.insert(self.key, value);
        }

        &mut self.parent[&self.key]
    }

    pub fn or_insert_with<F>(self, f: F) -> &'a mut V
        where F: FnOnce() -> V
    {
        if !self.parent.contains_key(&self.key) {
            self.parent.insert(self.key, f());
        }

        &mut self.parent[&self.key]
    }
}

impl<K, V> IntoIterator for DenseHashMap<K, V>
{
    type Item = (K, V);

    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.elements.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a DenseHashMap<K, V>
{
    type Item = &'a (K, V);

    type IntoIter = std::slice::Iter<'a, (K, V)>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.elements.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut DenseHashMap<K, V>
{
    type Item = &'a mut (K, V);

    type IntoIter = std::slice::IterMut<'a, (K, V)>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.elements.iter_mut()
    }
}
