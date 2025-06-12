// Re-export the types needed by the builder
pub use self::context::{
    ContextRef, DefaultEdgeContext, DefaultVertexContext, EdgeContext, VertexContext,
};
pub use self::control_flow::{EdgeControlFlow, VertexControlFlow};
pub use self::detour::{Detour, Waypoint};
pub use self::edges::Edges;
pub use self::empty::Empty;
pub use self::endpoints::{End, Endpoints};
pub use self::filter::{EdgeFilter, VertexFilter};
pub use self::take::{EdgeTake, VertexTake};
// No need to re-export the mutate_context types as they're not used externally
pub use self::probe::{EdgeProbe, VertexProbe};
pub use self::reduce::{EdgeReduce, VertexReduce};
pub use self::vertices::Vertices;
pub use self::vertices_by_id::VertexIter;

// These are the implementations for the builder methods
mod boxed;
mod collect;
mod context;
mod control_flow;
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
mod map;
mod mutate;
mod mutate_context;
mod probe;
mod reduce;
mod tail;
mod take;
mod vertices;
mod vertices_by_id;
