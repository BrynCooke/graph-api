// Re-export the types needed by the builder
pub use self::empty::Empty;
pub use self::filter::{EdgeFilter, VertexFilter};
pub use self::limit::{EdgeLimit, VertexLimit};
pub use self::probe::{EdgeProbe, VertexProbe};
pub use self::vertices::Vertices;
pub use self::edges::Edges;
pub use self::vertices_by_id::VertexIter;
pub use self::endpoints::{Endpoints, End};
pub use self::context::{ContextRef, DefaultEdgeContext, DefaultVertexContext, EdgeContext, VertexContext};
pub use self::detour::{Detour, Waypoint};

// These are the implementations for the builder methods
mod collect;
mod context;
mod count;
mod dbg;
mod default_context;
mod detour;
mod edges;
mod empty;
mod endpoints;
mod filter;
mod first;
mod fold;
mod head;
mod into_iter;
mod limit;
mod map;
mod mutate;
mod probe;
mod tail;
mod vertices;
mod vertices_by_id;