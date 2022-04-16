use std::ops::{Index, IndexMut};

use crate::base_veclist::BaseVecListIndex;
use crate::index::{IndexExt, ForwardIndex, BackwardIndex};

use super::VecList;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VecListIndex
{
    pub(super) base_index: BaseVecListIndex
}

impl<T> IndexExt<VecListIndex> for VecList<T>
{
    fn valid(&self, idx: &VecListIndex) -> bool
    {
        idx.base_index != self.base_list.begin()
            && idx.base_index != self.base_list.end()
            && self.base_list.valid(&idx.base_index)
    }
}

impl<T> ForwardIndex<VecListIndex> for VecList<T>
{
    fn begin(&self) -> VecListIndex { VecListIndex { base_index: self.base_list.next(self.base_list.begin()) } } 
    fn increment(&self, idx: &mut VecListIndex) { self.base_list.increment(&mut idx.base_index) }
}

impl<T> BackwardIndex<VecListIndex> for VecList<T>
{
    fn end(&self) -> VecListIndex { VecListIndex { base_index: self.base_list.prev(self.base_list.end()) } }
    fn decrement(&self, idx: &mut VecListIndex) { self.base_list.decrement(&mut idx.base_index) }
}

impl<T> Index<VecListIndex> for VecList<T>
{
    type Output = T;
    fn index(&self, index: VecListIndex) -> &Self::Output
    {
        assert!(self.valid(&index), "Invalid VecList index dereference");
        &self.base_list[index.base_index]
    }
}

impl<T> IndexMut<VecListIndex> for VecList<T>
{
    fn index_mut(&mut self, index: VecListIndex) -> &mut Self::Output
    {
        assert!(self.valid(&index), "Invalid VecList index dereference");
        &mut self.base_list[index.base_index]
    }
}
