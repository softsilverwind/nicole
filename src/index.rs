use std::ops::Index;

pub trait IndexExt<IndexType>: Index<IndexType>
{
    fn valid(&self, index: &IndexType) -> bool;
}

pub trait ForwardIndex<IndexType>: IndexExt<IndexType>
{
    fn begin(&self) -> IndexType;
    fn increment(&self, idx: &mut IndexType);
    fn next(&self, mut idx: IndexType) -> IndexType { self.increment(&mut idx); idx }
}

pub trait BackwardIndex<IndexType>: IndexExt<IndexType>
{
    fn end(&self) -> IndexType;
    fn decrement(&self, idx: &mut IndexType);
    fn prev(&self, mut idx: IndexType) -> IndexType { self.decrement(&mut idx); idx }
}

pub trait BidirectionalIndex<IndexType>: ForwardIndex<IndexType> + BackwardIndex<IndexType> {}
impl<IndexType, T> BidirectionalIndex<IndexType> for T where T: ForwardIndex<IndexType> + BackwardIndex<IndexType> {}
