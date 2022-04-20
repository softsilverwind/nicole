pub mod dense_hashset;
pub mod id_set;
pub mod identifier;
pub mod index;
pub mod veclist;

#[cfg(feature = "unsafe")] pub mod linkedlist;

pub use identifier::{IdLike, Identifier};
pub use index::{IndexExt, ForwardIndex, BackwardIndex, BidirectionalIndex};

pub use dense_hashset::DenseHashSet;
pub use id_set::IdSet;
pub use veclist::VecList;

#[cfg(feature = "unsafe")] pub use linkedlist::LinkedList;

mod standard_impls;
