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

/// An Index is a fast lookup mechanism for graph elements.
///
/// Indexes allow for efficient querying of graph elements based on different criteria.
/// Each index has a type and can be identified by an ordinal within its type.
pub trait Index
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// Returns the TypeId of the element being indexed.
    /// 
    /// Supported types are graph dependent, but all graph implementations support
    /// basic Rust types and strings.
    fn ty(&self) -> TypeId;

    /// Returns the ordinal number of this index.
    ///
    /// The ordinal uniquely identifies an index within its index type.
    fn ordinal(&self) -> usize;

    /// Returns the type of this index (Hash, Range, or FullText).
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
