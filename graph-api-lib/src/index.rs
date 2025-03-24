use std::any::TypeId;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// The type of index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IndexType {
    /// A hash index that supports lookup by value.
    Hash,
    /// A range index that supports range search
    Range,
    /// A full text index
    FullText,
}

impl Display for IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexType::Hash => write!(f, "hash"),
            IndexType::Range => write!(f, "range"),
            IndexType::FullText => write!(f, "full text"),
        }
    }
}

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

    /// The type of index
    fn index_type(&self) -> IndexType;
}

impl Index for () {
    fn ty(&self) -> TypeId {
        unimplemented!("index not implemented")
    }

    fn ordinal(&self) -> usize {
        unimplemented!("index not implemented")
    }

    fn index_type(&self) -> IndexType {
        unimplemented!("index not implemented")
    }
}
