pub mod index;
pub mod arraylist;

mod standard_impls;

pub use index::{
    IndexExt, ForwardIndex, BackwardIndex, BidirectionalIndex
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
