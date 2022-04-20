use super::{INVALID, LAST};

use super::{VecList, ERROR_MSG};

pub struct Iter<'a, T>
    where T: 'a
{
    veclist: &'a VecList<T>,
    index: usize
}

impl<'a, T> Iter<'a, T>
{
    pub fn new(veclist: &'a VecList<T>, index: usize) -> Self
    {
        Self { veclist, index }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: 'a
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.index {
            INVALID | LAST => None,
            n => {
                self.index = self.veclist.elements[n].next; 
                Some(self.veclist.elements[n].elem.as_ref().expect(ERROR_MSG))
            }
        }
    }
}

#[cfg(feature = "unsafe")]
pub struct IterMut<'a, T>
    where T: 'a
{
    veclist: &'a mut VecList<T>,
    index: usize
}

#[cfg(feature = "unsafe")]
impl<'a, T> IterMut<'a, T>
{
    pub fn new(veclist: &'a mut VecList<T>, index: usize) -> Self
    {
        Self { veclist, index }
    }
}

#[cfg(feature = "unsafe")]
impl<'a, T> Iterator for IterMut<'a, T>
    where T: 'a
{
    type Item = &'a mut T;
    fn next<'b>(&'b mut self) -> Option<Self::Item>
    {
        unsafe {
            match self.index {
                INVALID | LAST => None,
                n => {
                    self.index = self.veclist.elements[n].next;
                    let ret = self.veclist.elements[n].elem.as_mut().expect(ERROR_MSG);
                    Some(std::mem::transmute::<&'b mut T, &'a mut T>(ret))
                }
            }
        }
    }
}

pub struct IntoIter<T>
{
    veclist: VecList<T>,
    index: usize
}

impl<T> IntoIter<T>
{
    pub fn new(veclist: VecList<T>, index: usize) -> Self
    {
        Self { veclist, index }
    }
}

impl<T> Iterator for IntoIter<T>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.index {
            INVALID | LAST => None,
            n => {
                self.index = self.veclist.elements[n].next;
                Some(self.veclist.elements[n].elem.take().expect(ERROR_MSG))
            }
        }
    }
}

pub struct DrainIter<'a, T>
{
    veclist: &'a mut VecList<T>,
    index: usize
}

impl<'a, T> DrainIter<'a, T>
{
    pub fn new(veclist: &'a mut VecList<T>, index: usize) -> Self
    {
        Self { veclist, index }
    }
}

impl<'a, T> Iterator for DrainIter<'a, T>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.index {
            INVALID | LAST => None,
            n => {
                self.index = self.veclist.elements[n].next;
                self.veclist.elements[n].next = INVALID;
                self.veclist.elements[n].prev = INVALID;
                Some(self.veclist.elements[n].elem.take().expect(ERROR_MSG))
            }
        }
    }
}

impl<'a, T> Drop for DrainIter<'a, T>
{
    fn drop(&mut self)
    {
        self.for_each(|_|());
    }
}
