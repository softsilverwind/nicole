use std::collections::VecDeque;

use crate::index::{IndexExt, BackwardIndex, ForwardIndex};

impl<T> IndexExt<usize> for Vec<T>
{
    fn valid(&self, i: &usize) -> bool { *i < self.len() }
}

impl<T> ForwardIndex<usize> for Vec<T>
{
    fn begin(&self) -> usize { 0 }
    fn increment(&self, i: &mut usize) { *i += 1; }
    fn move_forward(&self, i: &mut usize, distance: usize) { *i += distance; }
}

impl<T> BackwardIndex<usize> for Vec<T>
{
    fn end(&self) -> usize { self.len().wrapping_sub(1) }
    fn decrement(&self, i: &mut usize) { *i = i.wrapping_sub(1); }
    fn move_backward(&self, i: &mut usize, distance: usize) { *i = i.wrapping_sub(distance); }
}

impl<T> IndexExt<usize> for VecDeque<T>
{
    fn valid(&self, i: &usize) -> bool { *i < self.len() }
}

impl<T> ForwardIndex<usize> for VecDeque<T>
{
    fn begin(&self) -> usize { 0 }
    fn increment(&self, i: &mut usize) { *i += 1; }
    fn move_forward(&self, i: &mut usize, distance: usize) { *i += distance; }
}

impl<T> BackwardIndex<usize> for VecDeque<T>
{
    fn end(&self) -> usize { self.len().wrapping_sub(1) }
    fn decrement(&self, i: &mut usize) { *i = i.wrapping_sub(1); }
    fn move_backward(&self, i: &mut usize, distance: usize) { *i = i.wrapping_sub(distance); }
}
