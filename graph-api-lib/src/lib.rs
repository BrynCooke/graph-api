// The types that are flagged by clippy generally can't be factored out as they use trait associated types.
#![allow(clippy::type_complexity)]
pub(crate) mod graph;

pub(crate) mod walker;

pub use element::Element;
pub use label::Label;

pub use graph::Direction;
pub use graph::EdgeReference;
pub use graph::EdgeReferenceMut;
pub use graph::Graph;
pub use graph::Id;
pub use graph::MutationListener;
pub use graph::Project;
pub use graph::ProjectMut;
pub use graph::Support;
pub use graph::Supported;
pub use graph::Unsupported;
pub use graph::VertexReference;
pub use graph::VertexReferenceMut;
pub use index::Index;
pub use value::Value;
pub use value::ValueOrRange;

pub use error::Error;
pub use search::edge::EdgeSearch;
pub use search::vertex::VertexSearch;
pub use walker::builder::EdgeWalkerBuilder;
pub use walker::builder::VertexWalkerBuilder;
pub use walker::builder::WalkerBuilder;
pub use walker::EdgeWalker;
pub use walker::VertexWalker;
pub use walker::Walker;
mod element;
mod error;
mod index;
mod label;
#[cfg(feature = "petgraph")]
pub mod petgraph;
mod search;
mod value;
