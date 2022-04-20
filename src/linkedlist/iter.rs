use super::{LinkedList, ListNode};

pub struct LinkedListIter<'a, T>
{
    pub(super) list: &'a LinkedList<T>,
    pub(super) list_ptr: *mut ListNode<T>
}

impl<'a, T> Iterator for LinkedListIter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        unsafe {
            if self.list_ptr != self.list.last {
                let elem = (*self.list_ptr).element.assume_init_ref();
                self.list_ptr = (*self.list_ptr).next;
                Some(elem)
            }
            else {
                None
            }
        }
    }
}
