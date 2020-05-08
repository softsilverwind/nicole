use std::ops::{Index, IndexMut};

use super::{BaseList, FIRST, LAST, INVALID};
use crate::index::{IndexExt, ForwardIndex, BackwardIndex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BaseListIndex
{
    pub(super) index: usize
}

impl<T> IndexExt<BaseListIndex> for BaseList<T>
{
    fn valid(&self, idx: &BaseListIndex) -> bool { idx.index != INVALID }
}

impl<T> ForwardIndex<BaseListIndex> for BaseList<T>
{
    fn begin(&self) -> BaseListIndex { BaseListIndex { index: FIRST } }
    fn increment(&self, idx: &mut BaseListIndex) { idx.index = self.next(idx.index); }
}

impl<T> BackwardIndex<BaseListIndex> for BaseList<T>
{
    fn end(&self) -> BaseListIndex { BaseListIndex { index: LAST } }
    fn decrement(&self, idx: &mut BaseListIndex) { idx.index = self.prev(idx.index); }
}


impl<T> Index<BaseListIndex> for BaseList<T>
{
    type Output = T;
    fn index(&self, index: BaseListIndex) -> &Self::Output
    {
        &self.elements[index.index].elem
    }
}

impl<T> IndexMut<BaseListIndex> for BaseList<T>
{
    fn index_mut(&mut self, index: BaseListIndex) -> &mut Self::Output
    {
        &mut self.elements[index.index].elem
    }
}
