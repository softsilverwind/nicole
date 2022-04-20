use std::iter::FromIterator;

#[cfg(feature = "unsafe")]
use std::cell::RefCell;

use crate::identifier::IdLike;

#[derive(Default, Clone)]
pub struct IdSet<T> {
    set: Vec<bool>,
    #[cfg(feature = "unsafe")]
    materialized: RefCell<Vec<T>>,
    #[cfg(not(feature = "unsafe"))]
    phantom: std::marker::PhantomData<T>
}

impl<T> IdSet<T>
    where T: IdLike,
{
    pub fn new() -> Self {
        Self {
            set: Vec::new(),
            #[cfg(feature = "unsafe")]
            materialized: RefCell::new(Vec::new()),
            #[cfg(not(feature = "unsafe"))]
            phantom: Default::default()
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let pos: usize = (*value).into();
        pos < self.set.len() && self.set[pos]
    }

    pub fn remove(&mut self, value: &T) {
        #[cfg(feature = "unsafe")]
        self.materialized.borrow_mut().clear();

        let pos: usize = (*value).into();

        if pos < self.set.len() {
            self.set[pos] = false;
        }
    }

    pub fn insert(&mut self, value: T) {
        #[cfg(feature = "unsafe")]
        self.materialized.borrow_mut().clear();

        let pos: usize = value.into();

        if pos >= self.set.len() {
            self.set.resize(pos + 1, false);
        }

        self.set[pos] = true;
    }

    #[cfg(feature = "unsafe")]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> + 'a
    {
        if self.materialized.borrow().is_empty() {
            *self.materialized.borrow_mut() = self.set
                .iter()
                .enumerate()
                .filter_map(|(id, &exists)| if exists { Some(T::from(id)) } else { None } )
                .collect();
        }

        // The idea here is that:
        // a. All other methods that mutate the RefCell should expect a &mut self
        // - Thus, before trying again to borrow_mut, all iteratos will have been dropped
        // - Thus, all element borrows returned from the unsafe iterator will have been dropped
        // b. Yielded elements are already tied to the IdSet lifetime, so no &mut self method can be called
        //    as long as a borrow on an element is alive
        // c. Iter should only borrow_mut on the *first* iteration, where there is no aliasing on &self.
        //    Subsequent iterators will not borrow_mut anyway.
        //
        // Of course, if we add another &self method that mutates the materialized field, UB will ensue.

        unsafe {
            self.materialized.try_borrow_unguarded().unwrap().iter()
        }
    }

    pub fn iter_copy<'a>(&'a self) -> impl Iterator<Item=T> + 'a
    {
        self.set
            .iter()
            .enumerate()
            .filter_map(|(id, &exists)| if exists { Some(T::from(id)) } else { None } )
    }
}

impl<T> FromIterator<T> for IdSet<T>
    where T: IdLike,
{
    fn from_iter<IT>(iter: IT) -> Self
    where
        IT: IntoIterator<Item = T>,
    {
        let mut ret = IdSet::new();
        for item in iter.into_iter() {
            ret.insert(item);
        }

        ret
    }
}

impl<T> Extend<T> for IdSet<T>
    where T: IdLike
{
    fn extend<IT>(&mut self, iter: IT)
    where
        IT: IntoIterator<Item = T>,
    {
        for x in iter {
            self.insert(x)
        }
    }
}

impl<T> IntoIterator for IdSet<T>
    where T: IdLike
{
    type Item = T;
    // TODO check if impl Iterator<Item=T> applies after https://github.com/rust-lang/rust/issues/63063
    // has been merged
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {   
        self.set
            .into_iter()
            .enumerate()
            .filter_map(|(id, exists)| if exists { Some(T::from(id)) } else { None } )
            .collect::<Vec<_>>()
            .into_iter()
    }   
}
