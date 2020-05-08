use std::mem;
use super::INVALID;

use super::BaseList;

pub struct Iter<'a, T>
    where T: 'a
{
    parent: &'a BaseList<T>,
    index: usize
}

impl<'a, T> Iter<'a, T>
{
    pub fn new(parent: &'a BaseList<T>, index: usize) -> Self
    {
        Self { parent, index }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: 'a
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.index {
            INVALID => None,
            n => {
                self.index = self.parent.next(n); 
                Some(&self.parent.elements[n].elem)
            }
        }
    }
}

pub struct IterMut<'a, T>
    where T: 'a
{
    parent: &'a mut BaseList<T>,
    index: usize
}

impl<'a, T> IterMut<'a, T>
{
    pub fn new(parent: &'a mut BaseList<T>, index: usize) -> Self
    {
        Self { parent, index }
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
    where T: 'a
{
    type Item = &'a mut T;
    fn next<'b>(&'b mut self) -> Option<Self::Item>
    {
        unsafe {
            match self.index {
                INVALID => None,
                n => {
                    self.index = self.parent.next(n);
                    let ret = &mut self.parent.elements[n].elem;
                    Some(mem::transmute::<&'b mut T, &'a mut T>(ret))
                }
            }
        }
    }
}

pub struct IntoIter<T>
{
    parent: BaseList<T>,
    index: usize
}

impl<T> IntoIter<T>
{
    pub fn new(parent: BaseList<T>, index: usize) -> Self
    {
        Self { parent, index }
    }
}

impl<T> Iterator for IntoIter<T>
    where T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.index {
            INVALID => None,
            n => {
                self.index = self.parent.next(n);
                Some(self.parent.elements[n].elem.clone())
            }
        }
    }
}
