use std::any::TypeId;
use std::fmt::Debug;
use std::hash::Hash;

/// An Index is a fast lookup to an element. This is
pub trait Index
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// The type of the element being indexed.
    /// Supported types are graph dependant, however all graphs support basic rust type and strings.
    fn ty(&self) -> TypeId;

    /// The index ordinal
    fn ordinal(&self) -> usize;

    /// If the index is ordered
    fn ordered(&self) -> bool;

    /// If the index is full text
    fn full_text(&self) -> bool;
}

impl Index for () {
    fn ty(&self) -> TypeId {
        unimplemented!("index not implemented")
    }

    fn ordinal(&self) -> usize {
        unimplemented!("index not implemented")
    }

    fn ordered(&self) -> bool {
        false
    }

    fn full_text(&self) -> bool {
        false
    }
}
