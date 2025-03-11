#![allow(clippy::type_complexity)]
#![doc = include_str!("../../README.md")]
// The types that are flagged by clippy generally can't be factored out as they use trait associated types.
mod element;
mod error;
mod graph;
mod index;
mod label;
mod search;
mod value;
mod walker;

pub use element::Element;
pub use error::Error;
pub use graph::Direction;
pub use graph::EdgeReference;
pub use graph::EdgeReferenceMut;
pub use graph::ElementId;
pub use graph::Graph;
pub use graph::MutationListener;
pub use graph::Project;
pub use graph::ProjectMut;
pub use graph::Support;
pub use graph::Supported;
pub use graph::Unsupported;
pub use graph::VertexReference;
pub use graph::VertexReferenceMut;
pub use index::Index;
pub use label::Label;
pub use search::edge::EdgeSearch;
pub use search::vertex::VertexSearch;
pub use value::Value;
pub use value::ValueRange;
pub use walker::builder::EdgeWalkerBuilder;
pub use walker::builder::VertexWalkerBuilder;
pub use walker::builder::WalkerBuilder;
pub use walker::EdgeWalker;
pub use walker::VertexWalker;
pub use walker::Walker;

#[cfg(feature = "petgraph")]
pub mod petgraph;
