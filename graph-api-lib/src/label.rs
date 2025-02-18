use crate::Index;
use std::fmt::Debug;
use std::hash::Hash;

/// A label for an `Element`. This can be either a vertex or an edge.
pub trait Label
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// Information about indexes for this label
    type Index: Eq + Copy + Hash + Debug + Index + 'static;

    /// All label variants
    fn variants() -> &'static [Self];

    /// The indexes associated with this label
    fn indexes(&self) -> &'static [Self::Index];

    /// A unique ordinal for this label
    fn ordinal(&self) -> usize;

    /// The name of the label
    fn name(&self) -> &'static str;
}

impl Label for () {
    type Index = ();

    /// The anonymous label
    fn variants() -> &'static [Self] {
        &[()]
    }

    fn indexes(&self) -> &'static [Self::Index] {
        &[]
    }

    fn ordinal(&self) -> usize {
        0
    }

    fn name(&self) -> &'static str {
        "<anonymous>"
    }
}
