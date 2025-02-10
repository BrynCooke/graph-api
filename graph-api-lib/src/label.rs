use std::fmt::Debug;
use std::hash::Hash;

/// A label for an `Element`. This can be either a vertex or an edge.
pub trait Label
where
    Self: Sized + Copy + Eq + Hash + Debug,
{
    /// All label variants
    fn variants() -> &'static [Self];

    /// A unique ordinal for this label
    fn ordinal(&self) -> usize;

    /// The name of the label
    fn name(&self) -> &'static str;
}

impl Label for () {
    /// The anonymous label
    fn variants() -> &'static [Self] {
        &[()]
    }

    fn ordinal(&self) -> usize {
        0
    }

    fn name(&self) -> &'static str {
        "<anonymous>"
    }
}
