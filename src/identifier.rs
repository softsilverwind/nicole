pub trait IdLike: Copy + Into<usize> + From<usize> + Eq + Ord
{
    fn null() -> Self;
    fn is_null(self) -> bool { self == Self::null() }
}

pub trait Identifier
{
    type Id: IdLike;

    fn id(&self) -> Self::Id;
}

impl<T> Identifier for T
    where T: IdLike
{
    type Id = Self;

    fn id(&self) -> Self::Id { *self }
}
