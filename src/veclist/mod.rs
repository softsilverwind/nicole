use crate::base_veclist::BaseVecList;

mod iter;
mod index;

pub use self::index::VecListIndex;
pub use self::iter::{Iter, IterMut, IntoIter};

#[derive(Clone, Debug)]
pub struct VecList<T>
{
    base_list: BaseVecList<T>
}

impl<T> VecList<T>
{
    pub fn new() -> Self
        where T: Default
    {
        Self {
            base_list: BaseVecList::new(Default::default(), Default::default())
        }
    }

    pub fn with_capacity(capacity: usize) -> Self
        where T: Default
    {
        Self {
            base_list: BaseVecList::with_capacity(Default::default(), Default::default(), capacity + 2)
        }
    }

    pub fn capacity(&self) -> usize
    {
        self.base_list.capacity() - 2
    }

    pub fn reserve(&mut self, additional: usize)
    {
        self.base_list.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize)
    {
        self.base_list.reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self)
        where T: Clone
    {
        self.base_list.shrink_to_fit()
    }

    pub fn insert(&mut self, index: VecListIndex, element: T)
    {
        self.base_list.insert(index.base_index, element)
    } 

    pub fn remove_between<F>(&mut self, start: VecListIndex, end: VecListIndex, f: F)
        where F: FnMut(&T)
    {
        self.base_list.remove_between(start.base_index, end.base_index, f)
    }

    pub fn remove<F>(&mut self, index: VecListIndex, f: F) where F: FnMut(&T)
    {
        self.base_list.remove(index.base_index, f)
    }

    pub fn push_back(&mut self, value: T)
    {
        self.base_list.push_back(value)
    }

    pub fn push_front(&mut self, value: T)
    {
        self.base_list.push_front(value)
    }

    pub fn pop_back<F>(&mut self, f: F)
        where F: FnMut(&T)
    {
        self.base_list.pop_back(f)
    }

    pub fn pop_front<F>(&mut self, f: F)
        where F: FnMut(&T)
    {
        self.base_list.pop_front(f)
    }

    pub fn clear<F>(&mut self, f: F)
        where F: FnMut(&T)
    {
        self.base_list.clear(f)
    }

    pub fn len(&self) -> usize
    {
        self.base_list.len() - 2
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T>
    {
        let mut iter = self.base_list.iter();
        iter.next();
        Iter::new(iter.peekable())
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T>
    {
        let mut iter = self.base_list.iter_mut();
        iter.next();
        IterMut::new(iter.peekable())
    }
}

impl<T> IntoIterator for VecList<T>
    where T: Clone
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T>
    {
        let mut iter = self.base_list.into_iter();
        iter.next();
        IntoIter::new(iter.peekable())
    }
}
