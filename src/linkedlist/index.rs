use uuid::Uuid;

use super::ListNode;

pub struct LinkedListIndex<T>
{
    pub(super) list_tag: Uuid,
    pub(super) node_tag: Uuid,
    pub(super) list_ptr: *mut ListNode<T>
}

impl<T> std::fmt::Debug for LinkedListIndex<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("LinkedListIndex {}")
    }
}

impl<T> Copy for LinkedListIndex<T>
{
}

impl<T> Clone for LinkedListIndex<T>
{
    fn clone(&self) -> Self
    {
        Self {
            list_tag: self.list_tag.clone(),
            node_tag: self.node_tag.clone(),
            list_ptr: self.list_ptr
        }
    }
}

impl<T> PartialEq for LinkedListIndex<T>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.list_tag == other.list_tag && self.node_tag == other.node_tag && self.list_ptr == other.list_ptr
    }
}

impl<T> Eq for LinkedListIndex<T>
{
}
