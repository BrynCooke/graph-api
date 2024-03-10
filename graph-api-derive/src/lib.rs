use crate::model::DeriveType;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) mod model;
mod render;

#[proc_macro_derive(VertexExt, attributes(index))]
pub fn vertex_ext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match model::Model::try_from(DeriveType::Vertex(&input)) {
        Ok(model) => model.into_vertex().into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(EdgeExt, attributes(index))]
pub fn edge_ext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match model::Model::try_from(DeriveType::Edge(&input)) {
        Ok(model) => model.into_edge().into(),
        Err(e) => e.into_compile_error().into(),
    }
}
