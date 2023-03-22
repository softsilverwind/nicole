pub mod dense;
pub mod identifier;
pub mod idlike;
pub mod index;
pub mod typedvec;
pub mod veclist;

#[cfg(feature = "unsafe")] pub mod linkedlist;

pub use identifier::{IdLike, Identifier};
pub use index::{IndexExt, ForwardIndex, BackwardIndex, BidirectionalIndex};

pub use veclist::VecList;
pub use dense::{map::DenseMap, set::DenseSet};
pub use idlike::{map::IdMap, set::IdSet};

#[cfg(feature = "unsafe")] pub use linkedlist::LinkedList;

mod standard_impls;
