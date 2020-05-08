use std::{
    fmt::{self, Debug, Formatter},
    usize
};
    
mod iter;
mod index;

pub use self::{
    iter::{Iter, IterMut, IntoIter},
    index::BaseListIndex
};

#[derive(Clone)]
struct Node<T>
{
    pub prev: usize,
    pub next: usize,
    pub elem: T
}

impl<T> Node<T>
{
    fn new(prev: usize, next: usize, elem: T) -> Self { Self { prev, next, elem } }
}

#[derive(Clone)]
pub struct BaseList<T>
{
    elements: Vec<Node<T>>,
    free: Vec<usize>
}

const INVALID: usize = usize::MAX;
const FIRST: usize = 0;
const LAST: usize = 1;

impl<T> BaseList<T>
{
    fn next(&self, i: usize) -> usize { self.elements[i].next }
    fn prev(&self, i: usize) -> usize { self.elements[i].prev }

    pub fn new(begin: T, end: T) -> Self
    {
        Self {
            elements: vec![Node::new(INVALID, LAST, begin), Node::new(FIRST, INVALID, end)],
            free: Vec::new()
        }
    }

    pub fn with_capacity(begin: T, end: T, capacity: usize) -> Self
    {
        let mut elements = Vec::with_capacity(capacity);
        elements.push(Node::new(INVALID, LAST, begin));
        elements.push(Node::new(FIRST, INVALID, end));

        Self {
            elements,
            free: Vec::new()
        }
    }

    pub fn capacity(&self) -> usize { self.elements.capacity() }
    pub fn reserve(&mut self, additional: usize) { self.elements.reserve(additional) }
    pub fn reserve_exact(&mut self, additional: usize) { self.elements.reserve_exact(additional) }

    pub fn shrink_to_fit(&mut self)
        where T: Clone
    {
        let len = self.elements.len();
        let free = self.free.len();
        let mut new_elems: Vec<Node<T>> = Vec::with_capacity(len - free);
        new_elems.push(self.elements[FIRST].clone());
        new_elems.push(self.elements[LAST].clone());

        let mut i = FIRST;
        while i != INVALID {
            new_elems.push(self.elements[i].clone());
            i = self.next(i);
        }

        new_elems[FIRST].next = LAST + 1;
        new_elems[LAST].prev = new_elems.len() - 1;

        self.elements = new_elems;
        self.free = Vec::new();
    }

    fn _insert(&mut self, next: usize, element: T)
    {
        assert!(next >= LAST, "Cannot insert before the first element or after the last element of a BaseList");
        let prev = self.prev(next);

        let elem = Node { prev, next, elem: element };

        // Let's see if we can't reuse some memory:
        let pos = match self.free.pop() {
            Some(x) => {
                self.elements[x] = elem;
                x
            },
            None => {
                self.elements.push(elem);
                self.elements.len() - 1
            }
        };

        self.elements[prev].next = pos;
        self.elements[next].prev = pos;

    }

    pub fn insert(&mut self, index: BaseListIndex, element: T)
    {
        self._insert(index.index, element);
    }

    fn _remove_between<F>(&mut self, start: usize, end: usize, mut f: F)
        where F: FnMut(&T)
    {
        assert!(start > LAST && end != INVALID, "Cannot remove the first or last element of a BaseList!");

        let mut i = start;
        while i != end && i != LAST {
            f(&self.elements[i].elem);
            self.free.push(i);
            i = self.next(i);
        }

        let start_prev = self.prev(start);

        self.elements[start_prev].next = end;
        self.elements[end].prev = start_prev;
    }

    pub fn remove_between<F>(&mut self, start: BaseListIndex, end: BaseListIndex, f: F)
        where F: FnMut(&T)
    {
        self._remove_between(start.index, end.index, f);
    }

    fn _remove<F>(&mut self, at: usize, f: F)
        where F: FnMut(&T)
    {
        let next = self.next(at);
        self._remove_between(at, next, f);
    }

    pub fn remove<F>(&mut self, index: BaseListIndex, f: F)
        where F: FnMut(&T)
    {
        self._remove(index.index, f);
    }

    pub fn push_back(&mut self, value: T)
    {
        self._insert(LAST, value);
    }

    pub fn push_front(&mut self, value: T)
    {
        let next = self.next(FIRST);
        self._insert(next, value);
    }

    pub fn pop_back<F>(&mut self, f: F)
        where F: FnMut(&T)
    {
        let at = self.prev(LAST);
        self._remove(at, f);
    }

    pub fn pop_front<F>(&mut self, f: F)
        where F: FnMut(&T)
    {
        let at = self.next(FIRST);
        self._remove(at, f);
    }

    pub fn clear<F>(&mut self, f: F)
        where F: FnMut(&T)
    {
        let first = self.next(FIRST);
        self._remove_between(first, LAST, f);
    }

    pub fn len(&self) -> usize
    {
        self.elements.len() - self.free.len()
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T>
    {
        Iter::new(self, FIRST)
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T>
    {
        IterMut::new(self, FIRST)
    }
}

impl<T> IntoIterator for BaseList<T>
    where T: Clone
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T>
    {
        IntoIter::new(self, FIRST)
    }
}


impl<T> Debug for BaseList<T>
    where T: Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "BaseList [ ")?;
        for node in self.iter() {
            write!(f, "{:?} ", node)?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}

