use std::collections::HashSet;

use uuid::Uuid;

use super::ListNode;

pub enum UuidSet
{
    Uuid(Uuid),
    Set(HashSet<Uuid>),
    Uninit
}

impl UuidSet
{
    pub fn matches(&self, rhs: &Uuid) -> bool
    {
        match self {
            UuidSet::Uuid(uuid) => uuid == rhs,
            UuidSet::Set(set) => set.contains(rhs),
            UuidSet::Uninit => panic!()
        }
    }

    pub fn new() -> Self
    {
        UuidSet::Uuid(Uuid::new_v4())
    }

    pub fn first(&self) -> Uuid
    {
        match self {
            UuidSet::Uuid(uuid) => *uuid,
            UuidSet::Set(set) => *set.iter().next().unwrap(),
            UuidSet::Uninit => panic!()
        }
    }

    pub fn merge(&mut self, rhs: &mut UuidSet)
    {
        let lhs = std::mem::replace(self, UuidSet::Uninit);
        let rhs = std::mem::replace(rhs, UuidSet::Uninit);

        let newset = match (lhs, rhs) {
            (UuidSet::Uuid(u1), UuidSet::Uuid(u2)) => { let mut s = HashSet::new(); s.insert(u1); s.insert(u2); s },
            (UuidSet::Uuid(u), UuidSet::Set(mut s)) | (UuidSet::Set(mut s), UuidSet::Uuid(u)) => { s.insert(u); s },
            (UuidSet::Set(s1), UuidSet::Set(s2)) => s1.union(&s2).copied().collect(),
            (_, _) => panic!()
        };

        *self = UuidSet::Set(newset);
    }
}

pub(super) struct FreeVec<T>(Vec<Vec<*mut ListNode<T>>>);

impl<T> FreeVec<T>
{
    pub fn pop(&mut self) -> Option<*mut ListNode<T>>
    {
        let mut free_node = None;
        while let Some(freevec) = self.0.last_mut() {
            if let Some(node) = freevec.pop() {
                free_node = Some(node);
                break;
            }
            else {
                self.0.pop();
            }
        }

        free_node
    }

    pub fn new() -> Self
    {
        FreeVec(Vec::new())
    }

    pub fn push(&mut self, ptr: *mut ListNode<T>)
    {
        if self.0.is_empty() {
            self.0.push(Vec::new());
        }

        self.0.last_mut().unwrap().push(ptr);
    }

    pub fn merge(&mut self, rhs: &mut FreeVec<T>)
    {
        self.0.append(&mut rhs.0);
    }

    pub unsafe fn clear(&mut self)
    {
        for vec in self.0.drain(..) {
            for free in vec {
                let _box = Box::from_raw(free);
            }
        }
    }
}
