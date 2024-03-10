use crate::model::{EnumVariants, Field, Model, Variant, VariantType};
use case::CaseExt;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

impl Model {
    pub(crate) fn into_edge(self) -> TokenStream {
        let trait_name = &self.extension_trait.ident;
        let enum_label = self.as_enum_label();
        let enum_index = self.as_enum_index();
        let impl_index = self.as_impl_index();
        let impl_element = self.as_impl_element();

        let projections: Vec<TokenStream> = self
            .variants
            .iter()
            .filter(|v| v.variant_type == VariantType::Named)
            .map(Variant::edge_projection)
            .collect();

        let nav_api: Vec<TokenStream> = self
            .extension_trait
            .enum_variants
            .iter()
            .map(EnumVariants::edge_api)
            .collect();

        let nav_impl: Vec<TokenStream> = self
            .extension_trait
            .enum_variants
            .iter()
            .map(EnumVariants::edge_implementation)
            .collect();

        let vis = &self.visibility;

        quote! {

            #enum_label
            #enum_index
            #impl_index
            #impl_element

            #(#projections)*

            #vis trait #trait_name<'graph, Mutability, Graph, Walker>
            where
                Walker: graph_api_lib::EdgeWalker<'graph, Graph = Graph>,
                Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
            {
                #(#nav_api)*
            }

            impl<'graph, Mutability, Graph, Walker> #trait_name<'graph, Mutability, Graph, Walker>
                for graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, Walker>
            where
                Walker: graph_api_lib::EdgeWalker<'graph, Graph = Graph>,
                Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
            {
                #(#nav_impl)*
            }
        }
    }

    pub(crate) fn into_vertex(self) -> TokenStream {
        let trait_name = &self.extension_trait.ident;
        let enum_label = self.as_enum_label();
        let enum_index = self.as_enum_index();
        let impl_index = self.as_impl_index();
        let impl_element = self.as_impl_element();

        let projections: Vec<TokenStream> = self
            .variants
            .iter()
            .filter(|v| v.variant_type == VariantType::Named)
            .map(Variant::vertex_projection)
            .collect();

        let nav_api: Vec<TokenStream> = self
            .extension_trait
            .enum_variants
            .iter()
            .map(EnumVariants::vertex_api)
            .collect();

        let nav_impl: Vec<TokenStream> = self
            .extension_trait
            .enum_variants
            .iter()
            .map(EnumVariants::vertex_implementation)
            .collect();

        let vis = &self.visibility;

        quote! {

            #enum_label
            #enum_index
            #impl_index
            #impl_element

            #(#projections)*

            #vis trait #trait_name<'graph, Mutability, Graph, Walker>
            where
                Walker: graph_api_lib::VertexWalker<'graph, Graph = Graph>,
                Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
            {
                #(#nav_api)*
            }

            impl<'graph, Mutability, Graph, Walker> #trait_name<'graph, Mutability, Graph, Walker>
                for graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, Walker>
            where
                Walker: graph_api_lib::VertexWalker<'graph, Graph = Graph>,
                Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge>,
            {
                #(#nav_impl)*
            }
        }
    }

    fn as_impl_index(&self) -> TokenStream {
        let index_ident = &self.index_ident;
        let index_count = self.variants.iter().fold(0, |acc, v| {
            acc + v.fields.iter().filter(|f| f.indexed).count()
        });
        let indexes: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::indexes)
            .collect::<Vec<_>>();
        let index_types: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::index_types)
            .collect::<Vec<_>>();

        let index_ordinals: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::index_ordinals)
            .collect::<Vec<_>>();

        let full_text: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::index_full_text)
            .collect::<Vec<_>>();

        let index_ordered: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::index_ordered)
            .collect::<Vec<_>>();

        if index_count == 0 {
            quote! {
                impl graph_api_lib::Index for #index_ident {
                    fn variants()-> &'static[#index_ident] {
                        static VARIANTS: [#index_ident; #index_count] = [#(#index_ident::#indexes),*];
                        &VARIANTS
                    }

                    fn ty(&self) -> core::any::TypeId {
                        unreachable!("this index enum has no variants")
                    }

                    fn ordinal(&self) -> usize {
                        unreachable!("this index enum has no variants")
                    }
                }
            }
        } else {
            quote! {
                 impl graph_api_lib::Index for #index_ident {
                    fn variants()-> &'static[#index_ident] {
                        static VARIANTS: [#index_ident; #index_count] = [#(#index_ident::#indexes),*];
                        &VARIANTS
                    }

                    fn ty(&self) -> core::any::TypeId {
                        match self {
                            #(#index_ident::#indexes => core::any::TypeId::of::<#index_types>()),*
                        }
                    }

                    fn ordinal(&self) -> usize {
                        match self {
                            #(#index_ident::#indexes => #index_ordinals),*
                        }
                    }

                    fn full_text(&self) -> bool {
                        match self {
                            #(#index_ident::#indexes => #full_text),*
                        }
                    }

                    fn ordered(&self) -> bool {
                        match self {
                            #(#index_ident::#indexes => #index_ordered),*
                        }
                    }
                }
            }
        }
    }

    fn as_impl_element(&self) -> TokenStream {
        let ident = &self.ident;
        let label_ident = &self.label_ident;
        let index_ident = &self.index_ident;
        let index_count = self.variants.iter().fold(0, |acc, v| {
            acc + v.fields.iter().filter(|f| f.indexed).count()
        });
        let variants: Vec<TokenStream> = self
            .variants
            .iter()
            .map(Variant::label_implementation)
            .collect::<Vec<_>>();

        let index_accessor: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::index_accessors)
            .collect::<Vec<_>>();

        let indexes: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::indexes)
            .collect::<Vec<_>>();

        let fn_value = if index_accessor.is_empty() {
            quote! {}
        } else {
            quote! {
                fn value(&self, index: &Self::Index) -> Option<graph_api_lib::Value> {
                    match (self, index) {
                        #(#index_accessor),*
                        (_,_)=> None
                    }
                }
            }
        };

        quote! {
            impl graph_api_lib::Element for #ident {
                type Label = #label_ident;
                type Index = #index_ident;
                fn label(&self) -> Self::Label {
                    match self {
                        #(#variants),*
                    }
                }

                #fn_value

                fn indexes() -> &'static[Self::Index] {
                   static INDEXES : [#index_ident; #index_count]= [
                       #(#index_ident::#indexes),*
                   ];
                   &INDEXES
                }
            }
        }
    }

    fn as_enum_label(&self) -> TokenStream {
        let vis = &self.visibility;
        let label_ident = &self.label_ident;
        let labels: Vec<TokenStream> = self
            .variants
            .iter()
            .map(Variant::labels)
            .collect::<Vec<_>>();
        let labels_count = labels.len();
        quote! {
            #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
            #vis enum #label_ident {
                #(#labels),*
            }

            impl graph_api_lib::Label for #label_ident {

                const COUNT : usize = #labels_count;

                fn ordinal(&self) -> usize {
                    *self as usize
                }
            }
        }
    }

    fn as_enum_index(&self) -> TokenStream {
        let vis = &self.visibility;
        let index_ident = &self.index_ident;
        let indexes: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::indexes)
            .collect::<Vec<_>>();

        let label_selectors = self
            .variants
            .iter()
            .map(Variant::label_selector)
            .collect::<Vec<_>>();
        let index_selectors: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::index_selectors)
            .collect::<Vec<_>>();

        let index_range_selectors: Vec<TokenStream> = self
            .variants
            .iter()
            .flat_map(Variant::index_range_selectors)
            .collect::<Vec<_>>();

        quote! {
            #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
            #vis enum #index_ident {
                #(#indexes),*
            }

            impl #index_ident {
                #(#label_selectors)*
                #(#index_selectors)*
                #(#index_range_selectors)*
            }
        }
    }
}

impl EnumVariants {
    fn edge_api(&self) -> TokenStream {
        match self {
            EnumVariants::Unit { all_ident, .. } => {
                quote! {
                  fn #all_ident(
                    self,
                  ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>;
                }
            }
            EnumVariants::NamedFields {
                filter_ident,
                all_ident,
                filter_param,
                ..
            } => {
                quote! {
                  fn #filter_ident<F: Fn(#filter_param<Graph::Edge>)->bool>(
                    self,
                        filter: F
                  ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>;

                  fn #all_ident(
                    self,
                  ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>;
                }
            }
            EnumVariants::UnnamedFields {
                filter_ident,
                all_ident,
                filter_params_types,
                ..
            } => {
                quote! {
                  fn #filter_ident<F: Fn(#(&#filter_params_types),*)->bool>(
                    self,
                        filter: F
                  ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>;

                  fn #all_ident(
                    self,
                  ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>;
                }
            }
        }
    }
    pub(crate) fn edge_implementation(&self) -> TokenStream {
        match self {
            EnumVariants::Unit {
                all_ident,
                target_ty,
                target_variant,
            } => {
                quote! {
                   fn #all_ident(
                        self,
                    ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context = Walker::Context>>
                    {
                        self.filter(|f| match graph_api_lib::EdgeReference::weight(f) {
                            #target_ty::#target_variant { .. } => true,
                            _ => false,
                        })
                    }
                }
            }
            EnumVariants::NamedFields {
                filter_ident,
                all_ident,
                target_ty,
                target_variant,
                filter_param,
            } => {
                quote! {
                    fn #filter_ident<F: Fn(#filter_param<Graph::Edge>)->bool>(
                        self,
                        filter: F
                    ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(move |f| {
                            if let Some(projection) = graph_api_lib::EdgeReference::project(f) {
                                (filter)(projection)
                            }
                            else {
                                false
                            }
                        })
                    }
                     fn #all_ident(
                        self,
                    ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(move |f| match graph_api_lib::EdgeReference::weight(f) {
                            #target_ty::#target_variant { .. } => true,
                            _ => false,
                        })
                    }
                }
            }
            EnumVariants::UnnamedFields {
                filter_ident,
                all_ident,
                target_ty,
                target_variant,
                filter_params_types,
            } => {
                let params = (0..filter_params_types.len())
                    .map(|p| format_ident!("param{}", p))
                    .collect::<Vec<_>>();
                quote! {
                    fn #filter_ident<F: Fn(#(&#filter_params_types),*)->bool>(
                        self,
                        filter: F
                    ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(move |f| match graph_api_lib::EdgeReference::weight(f) {
                            #target_ty::#target_variant ( #(#params),* ) => (filter)(#(#params),*),
                            _ => false,
                        })
                    }
                    fn #all_ident(
                        self,
                    ) -> graph_api_lib::EdgeWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::EdgeWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(move |f| match graph_api_lib::EdgeReference::weight(f) {
                            #target_ty::#target_variant ( .. ) => true,
                            _ => false,
                        })
                    }
                }
            }
        }
    }
}

impl EnumVariants {
    fn vertex_api(&self) -> TokenStream {
        match self {
            EnumVariants::Unit { all_ident, .. } => {
                quote! {
                  fn #all_ident(
                    self,
                  ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>;
                }
            }
            EnumVariants::NamedFields {
                filter_ident: ident,
                filter_param,
                all_ident,
                ..
            } => {
                quote! {
                    fn #all_ident(
                    self,
                  ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>;

                  fn #ident<F: Fn(#filter_param<Graph::Vertex>)->bool>(
                    self,
                        filter: F
                  ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>;
                }
            }
            EnumVariants::UnnamedFields {
                filter_ident: ident,
                filter_params_types,
                all_ident,
                ..
            } => {
                quote! {
                  fn #all_ident(
                    self,
                  ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>;

                  fn #ident<F: Fn(#(&#filter_params_types),*)->bool>(
                    self,
                        filter: F
                  ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>;
                }
            }
        }
    }
    fn vertex_implementation(&self) -> TokenStream {
        match self {
            EnumVariants::Unit {
                all_ident,
                target_ty,
                target_variant,
            } => {
                quote! {
                   fn #all_ident(
                        self,
                    ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(|f| match graph_api_lib::VertexReference::weight(f) {
                            #target_ty::#target_variant { .. } => true,
                            _ => false,
                        })
                    }
                }
            }
            EnumVariants::NamedFields {
                filter_ident,
                target_ty,
                target_variant,
                filter_param,
                all_ident,
                ..
            } => {
                quote! {
                    fn #all_ident(
                        self,
                    ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(|f| match graph_api_lib::VertexReference::weight(f) {
                            #target_ty::#target_variant { .. } => true,
                            _ => false,
                        })
                    }
                    fn #filter_ident<F: Fn(#filter_param<Graph::Vertex>)->bool>(
                        self,
                        filter: F
                    ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(move |f| {
                            if let Some(projection) = graph_api_lib::VertexReference::project(f) {
                                (filter)(projection)
                            }
                            else {
                                false
                            }
                        })
                    }
                }
            }
            EnumVariants::UnnamedFields {
                filter_ident,
                target_ty,
                target_variant,
                filter_params_types,
                all_ident,
                ..
            } => {
                let params = (0..filter_params_types.len())
                    .map(|p| format_ident!("param{}", p))
                    .collect::<Vec<_>>();
                quote! {
                    fn #all_ident(
                        self,
                    ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(|f| match graph_api_lib::VertexReference::weight(f) {
                            #target_ty::#target_variant { .. } => true,
                            _ => false,
                        })
                    }
                    fn #filter_ident<F: Fn(#(&#filter_params_types),*)->bool>(
                        self,
                        filter: F
                    ) -> graph_api_lib::VertexWalkerBuilder<'graph, Mutability, Graph, impl graph_api_lib::VertexWalker<'graph, Graph = Graph, Context=Walker::Context>>
                    {
                        self.filter(move |f| match graph_api_lib::VertexReference::weight(f) {
                            #target_ty::#target_variant ( #(#params),* ) => (filter)(#(#params),*),
                            _ => false,
                        })
                    }
                }
            }
        }
    }
}
impl Variant {
    fn label_implementation(&self) -> TokenStream {
        let ident = &self.element_ident;
        let variant = &self.ident;
        let label_ident = &self.label_ident;
        match &self.variant_type {
            VariantType::Unit => {
                quote! {#ident::#variant => #label_ident::#variant}
            }
            VariantType::Named => {
                quote! {#ident::#variant { .. } => #label_ident::#variant}
            }
            VariantType::Unnamed => {
                quote! {#ident::#variant(_) => #label_ident::#variant}
            }
        }
    }

    fn labels(&self) -> TokenStream {
        let variant = &self.ident;
        quote! {#variant}
    }

    fn indexes(&self) -> Vec<TokenStream> {
        self.indexed_fields()
            .map(|f| {
                let index_variant =
                    format_ident!("{}{}", self.ident, f.ident.to_string().to_camel());
                quote! {#index_variant}
            })
            .collect()
    }

    fn label_selector(&self) -> TokenStream {
        let vis = &self.visibility;
        let element_type = &self.element_type;
        let label_ident = &self.label_ident;
        let variant = &self.ident;
        let search_ident = &self.search_ident;
        let feature = format_ident!("Supports{}LabelIndex", element_type);

        let select_name = format_ident!("{}", self.ident.to_string().to_snake());
        quote! {#vis fn #select_name<'reference, Graph>() -> graph_api_lib::#search_ident<'reference, Graph>
            where
                Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge, #feature = graph_api_lib::Supported>,
                #element_type: graph_api_lib::Element<Index = Self>,
            {
            graph_api_lib::#search_ident::new().labelled(#label_ident::#variant)
        }}
    }

    fn index_selectors(&self) -> Vec<TokenStream> {
        let index_ident = &self.index_ident;
        let element_type = &self.element_type;
        let search_ident = &self.search_ident;

        self.indexed_fields()
            .map(|f| {
                let feature = if f.full_text {
                    format_ident!("Supports{}FullTextIndex", element_type)
                }
                else {
                    format_ident!("Supports{}Index", element_type)
                };

                let vis = &self.visibility;
                let index_variant =
                    format_ident!("{}{}", self.ident, f.ident.to_string().to_camel());
                let select_name =
                    format_ident!("{}_by_{}", self.ident.to_string().to_snake(), f.ident);
                let ty = f.ref_ty.clone();
                let label_ident = &self.label_ident;
                let variant = &self.ident;
                quote! {#vis fn #select_name<'reference, Graph>(value: #ty) -> graph_api_lib::#search_ident<'reference, Graph>
                    where
                        Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge, #feature = graph_api_lib::Supported>,
                        #element_type: graph_api_lib::Element<Index = Self>,
                    {
                    graph_api_lib::#search_ident::new().labelled(#label_ident::#variant).indexed(#index_ident::#index_variant, value)
                }}
            })
            .collect()
    }

    fn index_range_selectors(&self) -> Vec<TokenStream> {
        let index_ident = &self.index_ident;
        let element_type = &self.element_type;
        let search_ident = &self.search_ident;
        let feature = format_ident!("Supports{}OrderedIndex", element_type);

        self.indexed_fields()
            .filter(|f| f.ordered)
            .map(|f| {
                let vis = &self.visibility;
                let index_variant =
                    format_ident!("{}{}", self.ident, f.ident.to_string().to_camel());
                let select_name =
                    format_ident!("{}_by_{}_range", self.ident.to_string().to_snake(), f.ident);
                let ty = f.ref_ty.clone();
                let label_ident = &self.label_ident;
                let variant = &self.ident;
                quote! {#vis fn #select_name<'reference, Graph>(range: std::ops::Range<#ty>) -> graph_api_lib::#search_ident<'reference, Graph>
                    where
                        Graph: graph_api_lib::Graph<Vertex = Vertex, Edge = Edge, #feature = graph_api_lib::Supported>,
                        #element_type: graph_api_lib::Element<Index = Self>,
                    {
                    graph_api_lib::#search_ident::new().labelled(#label_ident::#variant).indexed(#index_ident::#index_variant, range)
                }}
            })
            .collect()
    }

    fn index_accessors(&self) -> Vec<TokenStream> {
        self.indexed_fields()
            .map(|f| {
                let element = &self.element_ident;
                let element_variant = &self.ident;
                let index_ident = &self.index_ident;
                let index_field = &f.ident;
                let index_variant =
                    format_ident!("{}{}", self.ident, f.ident.to_string().to_camel());
                quote! {(#element::#element_variant{#index_field,..}, #index_ident::#index_variant) => {Some((#index_field).into())}}
            })
            .collect()
    }

    fn index_types(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|f| {
                let ty = &f.ty;
                quote! {#ty}
            })
            .collect()
    }

    fn index_ordinals(&self) -> Vec<TokenStream> {
        self.indexed_fields()
            .enumerate()
            .map(|(idx, _)| {
                quote! {#idx}
            })
            .collect()
    }

    fn indexed_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter().filter(|f| f.indexed)
    }

    fn index_ordered(&self) -> Vec<TokenStream> {
        self.indexed_fields()
            .map(|idx| {
                let ty = &idx.ordered;
                quote! {#ty}
            })
            .collect()
    }

    fn index_full_text(&self) -> Vec<TokenStream> {
        self.indexed_fields()
            .map(|idx| {
                let ty = &idx.full_text;
                quote! {#ty}
            })
            .collect()
    }
}

impl Field {
    fn ident(&self) -> TokenStream {
        self.ident.to_token_stream()
    }

    fn field(&self) -> TokenStream {
        let ty = &self.ty;
        let ident = &self.ident;
        quote! {
            #ident: &'reference #ty
        }
    }

    fn mut_field(&self) -> TokenStream {
        let ty = &self.ty;
        let ident = &self.ident;
        quote! {
            #ident: &'reference mut #ty
        }
    }

    fn getter(&self) -> TokenStream {
        let vis = &self.visibility;
        let ident = &self.ident;
        let ty = &self.ref_ty;
        if self.ref_ty == self.ty {
            quote! {
                #vis fn #ident(&'reference self) -> #ty {
                    *self.#ident
                }
            }
        } else {
            quote! {
                #vis fn #ident(&'reference self) -> #ty {
                    self.#ident
                }
            }
        }
    }

    fn setter(&self) -> TokenStream {
        let vis = &self.visibility;
        let ident = &self.ident;
        let set_ident = format_ident!("set_{}", &self.ident);
        let ty = &self.ty;
        let index_ident = &self.index_ident;
        let index_variant = &self.index_variant;

        if self.indexed {
            quote! {
                #vis fn #set_ident(&mut self, value: #ty) {
                    self.__listener.update(#index_ident::#index_variant, (&*self.#ident).into(), (&value).into());
                    *self.#ident = value;
                }
            }
        } else {
            quote! {
                #vis fn #set_ident(&mut self, value: #ty) {
                    *self.#ident = value;
                }
            }
        }
    }
}

impl Variant {
    fn vertex_projection(&self) -> TokenStream {
        let vis = &self.visibility;
        let element_ident = &self.element_ident;
        let variant_ident = &self.ident;
        let ident = &self.ident;
        let mut_ident = &self.mut_ident;
        let fields = self.fields.iter().map(Field::ident).collect::<Vec<_>>();
        let struct_fields = self.fields.iter().map(Field::field);
        let struct_mut_fields = self.fields.iter().map(Field::mut_field);
        let fields_getters = self.fields.iter().map(Field::getter).collect::<Vec<_>>();
        let fields_setters = self.fields.iter().map(Field::setter);
        quote! {
            #vis struct #ident<'reference, Element> {
                _phantom: std::marker::PhantomData<Element>,
                #(#struct_fields),*
            }

            impl <'reference, Element> #ident<'reference, Element> {
                #(#fields_getters)*
            }

            #vis struct #mut_ident<'reference, Element, MutationListener> where
                Element: graph_api_lib::Element,
                MutationListener: graph_api_lib::MutationListener<'reference, Element>
            {
                _phantom: std::marker::PhantomData<Element>,
                __listener: MutationListener,
                #(#struct_mut_fields),*
            }

            impl <'reference, Element, MutationListener> #mut_ident<'reference, Element, MutationListener>
            where
                Element: graph_api_lib::Element<Index = VertexIndex>,
                MutationListener: graph_api_lib::MutationListener<'reference, Element>,
            {
                #(#fields_getters)*
                #(#fields_setters)*
            }

            impl<'reference> graph_api_lib::Project<'reference, #element_ident>
                for #ident<'reference, #element_ident>
            {
                fn project(weight: &'reference #element_ident) -> Option<Self> {
                    if let #element_ident::#variant_ident { #(#fields),* } = weight {
                        Some(#ident { _phantom: std::default::Default::default(), #(#fields),* })
                    }
                    else {
                        None
                    }
                }
            }

            impl<'reference, MutationListener> graph_api_lib::ProjectMut<'reference, #element_ident, MutationListener>
                for #mut_ident<'reference, #element_ident, MutationListener>
            where
                MutationListener: graph_api_lib::MutationListener<'reference, #element_ident>,
            {
                fn project_mut(weight: &'reference mut #element_ident, mutation_listener: MutationListener) -> Option<Self> {
                    if let #element_ident::#variant_ident { #(#fields),* } = weight {
                        Some(#mut_ident { _phantom: std::default::Default::default(), __listener: mutation_listener,  #(#fields),* })
                    }
                    else {
                        None
                    }
                }
            }
        }
    }

    fn edge_projection(&self) -> TokenStream {
        let vis = &self.visibility;
        let element_ident = &self.element_ident;
        let variant_ident = &self.ident;
        let ident = &self.ident;
        let mut_ident = &self.mut_ident;
        let fields = self.fields.iter().map(Field::ident).collect::<Vec<_>>();
        let struct_fields = self.fields.iter().map(Field::field);
        let struct_mut_fields = self.fields.iter().map(Field::mut_field);
        let fields_getters = self.fields.iter().map(Field::getter).collect::<Vec<_>>();
        let fields_setters = self.fields.iter().map(Field::setter);
        quote! {

            #vis struct #ident<'reference, Element> {
                _phantom: std::marker::PhantomData<Element>,
                #(#struct_fields),*
            }

            impl <'reference, Element> #ident<'reference, Element> {
                #(#fields_getters)*
            }

            #vis struct #mut_ident<'reference, Element, MutationListener> {
                _phantom: std::marker::PhantomData<Element>,
                __listener: MutationListener,
                #(#struct_mut_fields),*
            }

            impl <'reference, Element, MutationListener> #mut_ident<'reference, Element, MutationListener>
            {
                #(#fields_getters)*
                #(#fields_setters)*
            }

            impl<'reference> graph_api_lib::Project<'reference, #element_ident>
                for #ident<'reference, #element_ident>
            {
                fn project(weight: &'reference #element_ident) -> Option<Self> {
                    if let #element_ident::#variant_ident { #(#fields),* } = weight {
                        Some(#ident { _phantom: std::default::Default::default(), #(#fields),* })
                    }
                    else {
                        None
                    }
                }
            }

            impl<'reference, MutationListener> graph_api_lib::ProjectMut<'reference, #element_ident, MutationListener>
                for #mut_ident<'reference, #element_ident, MutationListener>
            where
                MutationListener: graph_api_lib::MutationListener<'reference, #element_ident>,
            {
                fn project_mut(weight: &'reference mut #element_ident, mutation_listener: MutationListener) -> Option<Self> {
                    if let #element_ident::#variant_ident { #(#fields),* } = weight {
                        Some(#mut_ident { _phantom: std::default::Default::default(), __listener: mutation_listener, #(#fields),* })
                    }
                    else {
                        None
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{DeriveType, Model};
    use insta::assert_snapshot;
    use quote::quote;
    use syn::DeriveInput;

    #[test]
    fn test_render_vertex() {
        let input = quote! {
            #[derive(VertexExt)]
            pub enum Vertex {
                Person {
                    #[index]
                    name: String,
                    #[index(ordered)]
                    age: u64,
                    #[index]
                    unique_id: Uuid,
                    #[index(ordered)]
                    username: String,
                    #[index(full_text)]
                    biography: String,
                },
                Project(Project),
                Rust,
            }
        };
        let parse: DeriveInput = syn::parse2(input).expect("parse");
        let model = Model::try_from(DeriveType::Vertex(&parse)).expect("model");
        let token_stream = model.into_vertex();
        let raw = token_stream.to_string();
        match syn::parse2(token_stream) {
            Ok(syntax_tree) => {
                let formatted = prettyplease::unparse(&syntax_tree);
                assert_snapshot!(formatted);
            }
            Err(_e) => {
                panic!("Failed to parse:\n{}", raw);
            }
        };
    }

    #[test]
    fn test_render_edge() {
        let input = quote! {
            #[derive(EdgeExt)]
            pub enum Edge {
                Knows { since: i32 },
                Created,
                Language(Language),
            }
        };
        let parse: DeriveInput = syn::parse2(input).expect("parse");
        let model = Model::try_from(DeriveType::Edge(&parse)).expect("model");
        let token_stream = model.into_edge();
        let raw = token_stream.to_string();
        match syn::parse2(token_stream) {
            Ok(syntax_tree) => {
                let formatted = prettyplease::unparse(&syntax_tree);
                assert_snapshot!(formatted);
            }
            Err(_e) => {
                panic!("Failed to parse:\n{}", raw);
            }
        };
    }
}
