use std::ops::{Index, IndexMut};

use super::{VecList, FIRST, LAST, INVALID, ERROR_MSG};
use crate::index::{IndexExt, ForwardIndex, BackwardIndex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VecListIndex
{
    pub(super) index: usize
}

impl<T> IndexExt<VecListIndex> for VecList<T>
{
    fn valid(&self, idx: &VecListIndex) -> bool { idx.index != INVALID }
}

impl<T> ForwardIndex<VecListIndex> for VecList<T>
{
    fn begin(&self) -> VecListIndex { VecListIndex { index: FIRST } }
    fn increment(&self, idx: &mut VecListIndex) { idx.index = self.elements[idx.index].next; }
}

impl<T> BackwardIndex<VecListIndex> for VecList<T>
{
    fn end(&self) -> VecListIndex { VecListIndex { index: LAST } }
    fn decrement(&self, idx: &mut VecListIndex) { idx.index = self.elements[idx.index].prev; }
}


impl<T> Index<VecListIndex> for VecList<T>
{
    type Output = T;
    fn index(&self, index: VecListIndex) -> &Self::Output
    {
        self.elements[index.index].elem.as_ref().expect(ERROR_MSG)
    }
}

impl<T> IndexMut<VecListIndex> for VecList<T>
{
    fn index_mut(&mut self, index: VecListIndex) -> &mut Self::Output
    {
        self.elements[index.index].elem.as_mut().expect(ERROR_MSG)
    }
}
