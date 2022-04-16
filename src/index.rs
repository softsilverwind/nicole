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
    fn move_forward(&self, idx: &mut IndexType, distance: usize) { for _ in 0..distance { self.increment(idx) } }
}

pub trait BackwardIndex<IndexType>: IndexExt<IndexType>
{
    fn end(&self) -> IndexType;
    fn decrement(&self, idx: &mut IndexType);
    fn prev(&self, mut idx: IndexType) -> IndexType { self.decrement(&mut idx); idx }
    fn move_backward(&self, idx: &mut IndexType, distance: usize) { for _ in 0..distance { self.decrement(idx) } }
}

pub trait BidirectionalIndex<IndexType>: ForwardIndex<IndexType> + BackwardIndex<IndexType>
{
    fn advance(&self, idx: &mut IndexType, distance: isize)
    {
        if distance > 0 {
            self.move_forward(idx, distance as usize)
        } else {
            self.move_backward(idx, (-distance) as usize)
        }
    }
}

impl<IndexType, T> BidirectionalIndex<IndexType> for T where T: ForwardIndex<IndexType> + BackwardIndex<IndexType> {}
