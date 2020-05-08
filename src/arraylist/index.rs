use std::ops::{Index, IndexMut};

use super::baselist::BaseListIndex;
use crate::index::{IndexExt, ForwardIndex, BackwardIndex};

use super::LinkedList;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LinkedListIndex
{
    pub(super) base_index: BaseListIndex
}

impl<T> IndexExt<LinkedListIndex> for LinkedList<T>
{
    fn valid(&self, idx: &LinkedListIndex) -> bool
    {
        idx.base_index != self.base_list.begin()
            && idx.base_index != self.base_list.end()
            && self.base_list.valid(&idx.base_index)
    }
}

impl<T> ForwardIndex<LinkedListIndex> for LinkedList<T>
{
    fn begin(&self) -> LinkedListIndex { LinkedListIndex { base_index: self.base_list.next(self.base_list.begin()) } } 
    fn increment(&self, idx: &mut LinkedListIndex) { self.base_list.increment(&mut idx.base_index) }
}

impl<T> BackwardIndex<LinkedListIndex> for LinkedList<T>
{
    fn end(&self) -> LinkedListIndex { LinkedListIndex { base_index: self.base_list.prev(self.base_list.end()) } }
    fn decrement(&self, idx: &mut LinkedListIndex) { self.base_list.decrement(&mut idx.base_index) }
}

impl<T> Index<LinkedListIndex> for LinkedList<T>
{
    type Output = T;
    fn index(&self, index: LinkedListIndex) -> &Self::Output
    {
        assert!(self.valid(&index), "Invalid LinkedList index dereference");
        &self.base_list[index.base_index]
    }
}

impl<T> IndexMut<LinkedListIndex> for LinkedList<T>
{
    fn index_mut(&mut self, index: LinkedListIndex) -> &mut Self::Output
    {
        assert!(self.valid(&index), "Invalid LinkedList index dereference");
        &mut self.base_list[index.base_index]
    }
}
