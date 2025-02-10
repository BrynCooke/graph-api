use std::any::TypeId;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Index
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// All the index variants
    fn variants() -> &'static [Self];

    /// The type of the index
    fn ty(&self) -> TypeId {
        unimplemented!("index not implemented")
    }

    /// The index ordinal
    fn ordinal(&self) -> usize {
        unimplemented!("index not implemented")
    }

    /// If the index is ordered
    fn ordered(&self) -> bool {
        false
    }

    /// If the index is full text
    fn full_text(&self) -> bool {
        false
    }
}

impl Index for () {
    fn variants() -> &'static [Self] {
        &[]
    }
}
