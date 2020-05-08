use std::iter::Peekable;
use super::baselist;

pub struct Iter<'a, T>
    where T: 'a
{
    base_iter: Peekable<baselist::Iter<'a, T>>
}

impl <'a, T> Iter<'a, T>
{
    pub fn new(base_iter: Peekable<baselist::Iter<'a, T>>) -> Self { Self { base_iter } }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: 'a
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {
        if let None = self.base_iter.peek() {
            None
        }
        else {
            self.base_iter.next()
        }
    }
}

pub struct IterMut<'a, T>
    where T: 'a
{
    base_iter: Peekable<baselist::IterMut<'a, T>>
}

impl <'a, T> IterMut<'a, T>
{
    pub fn new(base_iter: Peekable<baselist::IterMut<'a, T>>) -> Self { Self { base_iter } }
}

impl<'a, T> Iterator for IterMut<'a, T>
    where T: 'a
{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item>
    {
        if let None = self.base_iter.peek() {
            None
        }
        else {
            self.base_iter.next()
        }
    }
}

pub struct IntoIter<T>
    where T: Clone
{
    base_iter: Peekable<baselist::IntoIter<T>>
}

impl<T> IntoIter<T>
    where T: Clone
{
    pub fn new(base_iter: Peekable<baselist::IntoIter<T>>) -> Self { Self { base_iter } }
}

impl<T> Iterator for IntoIter<T>
    where T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
        if let None = self.base_iter.peek() {
            None
        }
        else {
            self.base_iter.next()
        }
    }
}
