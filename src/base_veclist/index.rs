use std::ops::{Index, IndexMut};

use super::{BaseVecList, FIRST, LAST, INVALID};
use crate::index::{IndexExt, ForwardIndex, BackwardIndex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BaseVecListIndex
{
    pub(super) index: usize
}

impl<T> IndexExt<BaseVecListIndex> for BaseVecList<T>
{
    fn valid(&self, idx: &BaseVecListIndex) -> bool { idx.index != INVALID }
}

impl<T> ForwardIndex<BaseVecListIndex> for BaseVecList<T>
{
    fn begin(&self) -> BaseVecListIndex { BaseVecListIndex { index: FIRST } }
    fn increment(&self, idx: &mut BaseVecListIndex) { idx.index = self.next(idx.index); }
}

impl<T> BackwardIndex<BaseVecListIndex> for BaseVecList<T>
{
    fn end(&self) -> BaseVecListIndex { BaseVecListIndex { index: LAST } }
    fn decrement(&self, idx: &mut BaseVecListIndex) { idx.index = self.prev(idx.index); }
}


impl<T> Index<BaseVecListIndex> for BaseVecList<T>
{
    type Output = T;
    fn index(&self, index: BaseVecListIndex) -> &Self::Output
    {
        &self.elements[index.index].elem
    }
}

impl<T> IndexMut<BaseVecListIndex> for BaseVecList<T>
{
    fn index_mut(&mut self, index: BaseVecListIndex) -> &mut Self::Output
    {
        &mut self.elements[index.index].elem
    }
}
