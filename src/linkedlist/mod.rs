use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    fmt::Debug
};

use uuid::Uuid;

use crate::index::{IndexExt, BackwardIndex, ForwardIndex};

pub mod index;
pub mod iter;

pub use index::LinkedListIndex;
pub use iter::LinkedListIter;

struct ListNode<T>
{
    element: MaybeUninit<T>,
    tag: Uuid,
    next: *mut ListNode<T>,
    prev: *mut ListNode<T>
}

impl<T> Default for ListNode<T>
{
    fn default() -> Self
    {
        Self {
            element: MaybeUninit::uninit(),
            tag: Uuid::new_v4(),
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut()
        }
    }
}

pub struct LinkedList<T>
{
    tag: Uuid,
    first: *mut ListNode<T>,
    last: *mut ListNode<T>,
    free: Vec<*mut ListNode<T>>,
}

#[derive(Debug)]
pub enum IndexError
{
    ListTagMismatch,
    ElemTagMismatch,
    OutOfBounds
}

impl<T> LinkedList<T>
{
    fn try_validate_index(&self, index: &LinkedListIndex<T>) -> Result<(), IndexError>
    {
        if self.tag != index.list_tag {
            Err(IndexError::ListTagMismatch)
        }
        else if index.list_ptr.is_null() || index.list_ptr == self.first || index.list_ptr == self.last {
            Err(IndexError::OutOfBounds)
        }
        // An index that has been created and belongs to this list should always point to valid memory,
        // due to preconditions. Modulo any bugs, of course.
        else if index.node_tag != unsafe { (*index.list_ptr).tag } {
            Err(IndexError::ElemTagMismatch)
        }
        else {
            Ok(())
        }
    }

    fn validate_index(&self, index: &LinkedListIndex<T>)
    {
        self.try_validate_index(index).unwrap();
    }

    pub fn new() -> Self
    {
        unsafe {
            let first = Box::into_raw(Box::new(ListNode::default()));
            let last = Box::into_raw(Box::new(ListNode::default()));
            (*first).next = last;
            (*last).prev = first;

            LinkedList {
                tag: Uuid::new_v4(),
                first,
                last,
                free: Vec::new()
            }
        }
    }

    // Clears the free vector. Invalidates all indices.
    pub fn gc(&mut self)
    {
        unsafe {
            let _boxes: Vec<_> = self.free.drain(..).map(|x| Box::from_raw(x)).collect();
        }
        self.tag = Uuid::new_v4();
        self.free.shrink_to_fit();
    }

    unsafe fn new_node(&mut self, element: T, prev: *mut ListNode<T>, next: *mut ListNode<T>) -> *mut ListNode<T>
    {
        let list_node = self.free.pop().unwrap_or_else(|| Box::into_raw(Box::new(Default::default())));

        *list_node = ListNode {
            element: MaybeUninit::new(element),
            tag: Uuid::new_v4(),
            next,
            prev
        };

        return list_node;
    }

    // Insert does not invalidate any indices
    pub fn insert(&mut self, index: LinkedListIndex<T>, element: T)
    {
        self.validate_index(&index);

        unsafe {
            let prev = (*index.list_ptr).prev;
            let next = index.list_ptr;
            let new_node = self.new_node(element, prev, next);

            (*prev).next = new_node;
            (*next).prev = new_node;
        }
    }

    // Remove invalidates the indices that point to this element
    pub fn remove(&mut self, index: LinkedListIndex<T>) -> (T, LinkedListIndex<T>)
    {
        self.validate_index(&index);

        unsafe {
            let next_index = self.next(index);

            let ret = (*index.list_ptr).element.assume_init_read();
            (*index.list_ptr).element = MaybeUninit::uninit();
            let next = (*index.list_ptr).next;
            let prev = (*index.list_ptr).prev;
            (*prev).next = next;
            (*next).prev = prev;

            (*index.list_ptr).tag = Uuid::new_v4();
            self.free.push(index.list_ptr);

            (ret, next_index)
        }
    }

    pub fn len(&self) -> usize
    {
        self.iter().count()
    }

    pub fn iter(&self) -> impl Iterator<Item=&T>
    {
        unsafe {
            LinkedListIter {
                list: self,
                list_ptr: (*self.first).next
            }
        }
    }

    pub fn push_back(&mut self, element: T)
    {
        unsafe {
            let prev = (*self.last).prev;
            let next = self.last;

            let new_node = self.new_node(element, prev, next);
            (*prev).next = new_node;
            (*next).prev = new_node;
        }
    }
}

impl<T> Index<LinkedListIndex<T>> for LinkedList<T>
{
    type Output = T;
    fn index(&self, index: LinkedListIndex<T>) -> &Self::Output
    {
        self.validate_index(&index);

        unsafe {
            (*index.list_ptr).element.assume_init_ref()
        }
    }
}
impl<T> IndexMut<LinkedListIndex<T>> for LinkedList<T>
{
    fn index_mut(&mut self, index: LinkedListIndex<T>) -> &mut Self::Output
    {
        self.validate_index(&index);

        unsafe {
            (*index.list_ptr).element.assume_init_mut()
        }
    }
}

impl<T> IndexExt<LinkedListIndex<T>> for LinkedList<T>
{
    fn valid(&self, index: &LinkedListIndex<T>) -> bool
    {
        self.try_validate_index(index).is_ok()
    }
}

impl<T> ForwardIndex<LinkedListIndex<T>> for LinkedList<T>
{
    fn begin(&self) -> LinkedListIndex<T>
    {
        unsafe {
            let list_ptr = (*self.first).next;

            LinkedListIndex {
                list_tag: self.tag,
                node_tag: (*list_ptr).tag,
                list_ptr
            }
        }
    }

    fn increment(&self, index: &mut LinkedListIndex<T>)
    {
        unsafe {
            match self.try_validate_index(index) {
                Ok(()) => {
                    let next = (*index.list_ptr).next;
                    index.list_ptr = next;
                    index.node_tag = (*next).tag;
                },
                Err(IndexError::OutOfBounds) => (),
                other_err => other_err.unwrap()
            }
        }
    }
}

impl<T> BackwardIndex<LinkedListIndex<T>> for LinkedList<T>
{
    fn end(&self) -> LinkedListIndex<T>
    {
        unsafe {
            let list_ptr = (*self.last).prev;

            LinkedListIndex {
                list_tag: self.tag,
                node_tag: (*list_ptr).tag,
                list_ptr
            }
        }
    }

    fn decrement(&self, index: &mut LinkedListIndex<T>)
    {
        unsafe {
            match self.try_validate_index(index) {
                Ok(()) => {
                    let prev = (*index.list_ptr).prev;
                    index.list_ptr = prev;
                    index.node_tag = (*prev).tag;
                },
                Err(IndexError::OutOfBounds) => {},
                other_err => other_err.unwrap()
            }
        }
    }
}

impl<T> Debug for LinkedList<T>
    where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        unsafe {
            f.write_fmt(format_args!("{:?} <-> {:?}\n", self.first, self.last))?;

            let mut curr = (*self.first).next;

            while curr != self.last {
                f.write_fmt(format_args!("At {:?}, [{:?}], {:?} <-> {:?}\n", curr, (*curr).element.assume_init_ref(), (*curr).next, (*curr).prev))?;
                curr = (*curr).next;
            }
        }
        Ok(())
    }
}

impl<T> Clone for LinkedList<T>
    where T: Clone
{
    fn clone(&self) -> Self
    {
        let mut ret = Self::new();

        for element in self.iter() {
            ret.push_back(element.clone());
        }

        ret
    }
}

impl<T> Drop for LinkedList<T>
{
    fn drop(&mut self)
    {
        self.gc();

        unsafe {
            let mut curr = (*self.first).next;

            let _box = Box::from_raw(self.first);

            while curr != self.last {
                let next = (*curr).next;
                
                (*curr).element.assume_init_drop();
                let _box = Box::from_raw(curr);

                curr = next;
            }
        }
    }
}
