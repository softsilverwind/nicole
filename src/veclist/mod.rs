use std::{
    fmt::{self, Debug, Formatter},
    usize,
    ops::{Bound, RangeBounds},
    iter::FromIterator
};
    
mod iter;
mod index;

pub use self::{
    iter::{Iter, IntoIter, DrainIter},
    index::VecListIndex
};

#[derive(Clone)]
struct Node<T>
{
    pub prev: usize,
    pub next: usize,
    pub elem: Option<T>
}

impl<T> Node<T>
{
    fn new(prev: usize, next: usize, elem: T) -> Self { Self { prev, next, elem: Some(elem) } }
    fn new_none(prev: usize, next: usize) -> Self { Self { prev, next, elem: None } }
}

#[derive(Clone)]
pub struct VecList<T>
{
    elements: Vec<Node<T>>,
    free: Vec<usize>
}

const INVALID: usize = usize::MAX;
const FIRST: usize = 0;
const LAST: usize = 1;
const ERROR_MSG: &'static str = "Internal error on nicole::VecList";

impl<T> VecList<T>
{
    pub fn new() -> Self
    {
        Self {
            elements: vec![Node::new_none(INVALID, LAST), Node::new_none(FIRST, INVALID)],
            free: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self
    {
        let mut elements = Vec::with_capacity(capacity + 2);
        elements.push(Node::new_none(INVALID, LAST));
        elements.push(Node::new_none(FIRST, INVALID));

        Self {
            elements,
            free: Vec::new()
        }
    }

    pub fn capacity(&self) -> usize { self.elements.capacity() - 2 }
    pub fn reserve(&mut self, additional: usize) { self.elements.reserve(additional) }
    pub fn reserve_exact(&mut self, additional: usize) { self.elements.reserve_exact(additional) }

    pub fn gc(&mut self)
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
            i = self.elements[i].next;
        }

        new_elems[FIRST].next = LAST + 1;
        new_elems[LAST].prev = new_elems.len() - 1;

        self.elements = new_elems;
        self.free = Vec::new();
    }

    fn _insert(&mut self, next: usize, element: T)
    {
        assert!(next >= LAST, "Cannot insert before the first element or after the last element of a VecList");
        let prev = self.elements[next].prev;

        let elem = Node::new(prev, next, element);

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

    pub fn insert(&mut self, next: VecListIndex, element: T)
    {
        self._insert(next.index, element);
    }

    pub fn drain(&mut self, range: impl RangeBounds<VecListIndex>) -> DrainIter<T>
    {
        let start = match range.start_bound() {
            Bound::Included(x) => x.index,
            Bound::Excluded(x) => self.elements[x.index].next,
            Bound::Unbounded => self.elements[FIRST].next,
        };

        let end = match range.end_bound() {
            Bound::Included(x) => self.elements[x.index].next,
            Bound::Excluded(x) => x.index,
            Bound::Unbounded => LAST
        };

        assert!(start > LAST && end != INVALID, "{}", ERROR_MSG);
        if start == end {
            return DrainIter::new(self, INVALID);
        }

        let mut i = start;
        while i != end && i != LAST {
            self.free.push(i);
            i = self.elements[i].next;
        }

        let start_prev = self.elements[start].prev;
        let i_prev = self.elements[i].prev;

        self.elements[start_prev].next = i;
        self.elements[i].prev = start_prev;

        self.elements[start].prev = INVALID;
        self.elements[i_prev].next = INVALID;

        DrainIter::new(self, start)
    }

    fn _remove(&mut self, index: usize) -> T
    {
        let ret = self.elements[index].elem.take();
        self.free.push(index);

        let prev = self.elements[index].prev;
        let next = self.elements[index].next;

        self.elements[prev].next = next;
        self.elements[next].prev = prev;

        self.elements[index].next = INVALID;
        self.elements[index].prev = INVALID;

        ret.expect(ERROR_MSG)
    }

    pub fn remove(&mut self, index: VecListIndex) -> T
    {
        self._remove(index.index)
    }

    pub fn push_back(&mut self, value: T)
    {
        self._insert(LAST, value);
    }

    pub fn push_front(&mut self, value: T)
    {
        let next = self.elements[FIRST].next;
        self._insert(next, value);
    }

    pub fn pop_back(&mut self) -> T
    {
        let at = self.elements[LAST].prev;
        self._remove(at)
    }

    pub fn pop_front(&mut self) -> T
    {
        let at = self.elements[FIRST].next;
        self._remove(at)
    }

    pub fn clear(&mut self)
    {
        self.elements.clear();
        self.free.clear();
    }

    pub fn len(&self) -> usize
    {
        self.elements.len() - self.free.len() - 2
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T>
    {
        Iter::new(self, self.elements[FIRST].next)
    }

    #[cfg(feature = "unsafe")]
    pub fn iter_mut<'a>(&'a mut self) -> iter::IterMut<'a, T>
    {
        iter::IterMut::new(self, self.elements[FIRST].next)
    }
}

impl<T> IntoIterator for VecList<T>
    where T: Clone
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T>
    {
        IntoIter::new(self, FIRST)
    }
}

impl<T> Debug for VecList<T>
    where T: Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let len = self.len();

        write!(f, "VecList [")?;
        for (i, node) in self.iter().enumerate() {
            write!(f, "{:?}", node)?;
            if i < len - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<ItemT> FromIterator<ItemT> for VecList<ItemT>
{
    fn from_iter<IteratorT>(iter: IteratorT) -> Self
        where IteratorT: IntoIterator<Item = ItemT>
    {
        let mut list = VecList::new();
        for item in iter {
            list.push_back(item);
        }
        list
    }
}
