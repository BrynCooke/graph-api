use case::CaseExt;
use quote::format_ident;
use std::ops::Deref;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident, Lifetime, Type, Visibility, parse_quote};

#[cfg_attr(test, derive(Debug))]
pub(crate) struct Model {
    pub(crate) visibility: Visibility,
    pub(crate) ident: Ident,
    pub(crate) label_ident: Ident,
    pub(crate) index_ident: Ident,
    pub(crate) variants: Vec<Variant>,
    pub(crate) extension_trait: ExtensionTrait,
}

#[cfg_attr(test, derive(Debug))]
pub(crate) struct Variant {
    pub(crate) visibility: Visibility,
    pub(crate) element_ident: Ident,
    pub(crate) ident: Ident,
    pub(crate) mut_ident: Ident,
    pub(crate) label_ident: Ident,
    pub(crate) index_ident: Ident,
    pub(crate) variant_type: VariantType,
    pub(crate) fields: Vec<Field>,
    pub(crate) element_type: Ident,
    pub(crate) projection_module: Ident,
    pub(crate) search_ident: Ident,
}

#[cfg_attr(test, derive(Debug))]
pub(crate) struct Field {
    pub(crate) visibility: Visibility,
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) index_ident: Ident,
    pub(crate) index_variant: Ident,
    pub(crate) indexed: bool,
    pub(crate) ordered: bool,
    pub(crate) full_text: bool,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Eq, PartialEq)]
pub(crate) enum VariantType {
    Unit,
    Named,
    Unnamed,
}

#[cfg_attr(test, derive(Debug))]
pub(crate) struct ExtensionTrait {
    pub(crate) ident: Ident,
    pub(crate) enum_variants: Vec<EnumVariants>,
}

#[cfg_attr(test, derive(Debug))]
pub(crate) enum EnumVariants {
    Unit {
        all_ident: Ident,
        target_ty: Ident,
        target_variant: Ident,
    },
    NamedFields {
        filter_ident: Ident,
        all_ident: Ident,
        target_ty: Ident,
        target_variant: Ident,
        filter_param: Ident,
    },
    UnnamedFields {
        filter_ident: Ident,
        all_ident: Ident,
        target_ty: Ident,
        target_variant: Ident,
        filter_params_types: Vec<Type>,
    },
}

pub(crate) enum DeriveType<'a> {
    Vertex(&'a DeriveInput),
    Edge(&'a DeriveInput),
}
impl Deref for DeriveType<'_> {
    type Target = DeriveInput;

    fn deref(&self) -> &Self::Target {
        match self {
            DeriveType::Vertex(input) | DeriveType::Edge(input) => input,
        }
    }
}

impl TryFrom<DeriveType<'_>> for Model {
    type Error = syn::Error;

    fn try_from(value: DeriveType) -> Result<Self, Self::Error> {
        let mut errors = Vec::new();
        let mut model = Model {
            visibility: value.vis.clone(),
            ident: value.ident.clone(),
            label_ident: format_ident!("{}Label", value.ident),
            index_ident: format_ident!("{}Index", value.ident),
            variants: vec![],
            extension_trait: ExtensionTrait {
                ident: format_ident!("{}Ext", value.ident),
                enum_variants: vec![],
            },
        };
        let element_type = match value {
            DeriveType::Vertex(_) => {
                format_ident!("Vertex")
            }
            DeriveType::Edge(_) => {
                format_ident!("Edge")
            }
        };

        model.variants = match &value.data {
            Data::Enum(data) => {
                data.variants
                    .iter()
                    .map(|variant| Variant {
                        visibility: model.visibility.clone(),
                        element_ident: model.ident.clone(),
                        ident: variant.ident.clone(),
                        mut_ident: format_ident!("{}Mut", variant.ident),
                        label_ident: model.label_ident.clone(),
                        index_ident: model.index_ident.clone(),
                        variant_type: match &variant.fields {
                            Fields::Named(_) => VariantType::Named,
                            Fields::Unnamed(_) => VariantType::Unnamed,
                            Fields::Unit => VariantType::Unit,
                        },
                        element_type: element_type.clone(),
                        projection_module: format_ident!("__{}_projection_{}_{}", element_type.to_string().to_snake(), model.ident.to_string().to_snake(), variant.ident.to_string().to_snake()),
                        search_ident: match value {
                            DeriveType::Vertex(_) => {format_ident!("VertexSearch")}
                            DeriveType::Edge(_) => {format_ident!("EdgeSearch")}
                        },
                        fields: match &variant.fields {
                            Fields::Named(fields) => fields
                                .named
                                .iter()
                                .map(|field| {
                                    let mut field_model = Field {
                                        visibility: model.visibility.clone(),
                                        ident: field
                                            .ident
                                            .as_ref()
                                            .expect("named field has ident")
                                            .clone(),
                                        ty: field.ty.clone(),
                                        ordered: false,
                                        full_text: false,
                                        indexed: false,
                                        index_ident: model.index_ident.clone(),
                                        index_variant: format_ident!("{}{}", variant.ident, field.ident.as_ref().expect("field must have name").to_string().to_camel()),
                                    };
                                    if let Some(attr) = field
                                        .attrs
                                        .iter()
                                        .find(|attr| attr.path().is_ident("index"))
                                    {
                                        field_model.indexed = true;
                                        let _ = attr.parse_nested_meta(|m| {
                                            if m.path.is_ident("ordered") {
                                                field_model.ordered = true;
                                            } else if m.path.is_ident("full_text") {
                                                if field.ty != parse_quote!(String) {
                                                    errors.push(syn::Error::new_spanned(
                                                        m.path,
                                                        "full_text index can only be used on String fields",
                                                    ));
                                                }
                                                field_model.full_text = true;
                                            } else {
                                                let ident = m.path.get_ident().cloned().unwrap_or_else(||format_ident!("<unknown>"));
                                                errors.push(syn::Error::new_spanned(
                                                    m.path,
                                                    format!("unknown index attribute {}", ident)),
                                                );
                                            }
                                            Ok(())
                                        });
                                    }
                                    field_model
                                })
                                .collect(),
                            Fields::Unnamed(_) => vec![], // Unnamed fields can't have indexes
                            Fields::Unit => vec![],       // Unit variants can't have indexes
                        },
                    })
                    .collect()
            }
            _ => {
                return Err(syn::Error::new(
                    value.span(),
                    "derive Vertex may only be used on an enum",
                ));
            }
        };

        model.extension_trait.enum_variants = match &value.data {
            Data::Enum(data) => data
                .variants
                .iter()
                .map(|variant| {
                    let name = variant.ident.to_string().to_snake();
                    let all_ident = format_ident!("filter_{}", name);
                    let filter_ident = format_ident!("filter_by_{}", name);
                    match &variant.fields {
                        Fields::Named(_fields) => EnumVariants::NamedFields {
                            filter_ident,
                            all_ident,
                            target_ty: value.ident.clone(),
                            target_variant: variant.ident.clone(),
                            filter_param: format_ident!("{}", variant.ident),
                        },
                        Fields::Unnamed(fields) => EnumVariants::UnnamedFields {
                            filter_ident,
                            all_ident,
                            target_ty: value.ident.clone(),
                            target_variant: variant.ident.clone(),
                            filter_params_types: fields
                                .unnamed
                                .iter()
                                .map(|f| f.ty.clone())
                                .collect(),
                        },
                        Fields::Unit => EnumVariants::Unit {
                            all_ident,
                            target_ty: value.ident.clone(),
                            target_variant: variant.ident.clone(),
                        },
                    }
                })
                .collect(),
            _ => {
                return Err(syn::Error::new(
                    value.span(),
                    "derive Vertex may only be used on an enum",
                ));
            }
        };
        if !errors.is_empty() {
            return Err(errors
                .into_iter()
                .reduce(|mut l, r| {
                    l.combine(r);
                    l
                })
                .expect("at least one error"));
        }
        Ok(model)
    }
}

pub(crate) fn ref_type(ty: &Type, lifetime: Option<Lifetime>) -> Type {
    if *ty == parse_quote!(String) {
        return match lifetime {
            None => {
                parse_quote!(&str)
            }
            Some(lifetime) => {
                parse_quote!(&#lifetime str)
            }
        };
    } else if *ty != parse_quote!(usize)
        && *ty != parse_quote!(u8)
        && *ty != parse_quote!(i8)
        && *ty != parse_quote!(u16)
        && *ty != parse_quote!(i16)
        && *ty != parse_quote!(u32)
        && *ty != parse_quote!(i32)
        && *ty != parse_quote!(f32)
        && *ty != parse_quote!(u64)
        && *ty != parse_quote!(i64)
        && *ty != parse_quote!(f64)
        && *ty != parse_quote!(u128)
        && *ty != parse_quote!(i128)
        && *ty != parse_quote!(Uuid)
    {
        return match lifetime {
            None => {
                parse_quote!(&#ty)
            }
            Some(lifetime) => {
                parse_quote!(&#lifetime #ty)
            }
        };
    }
    ty.clone()
}

#[cfg(test)]
mod test {
    use crate::model::{DeriveType, Model};
    use insta::assert_debug_snapshot;
    use quote::quote;
    use syn::DeriveInput;

    #[test]
    fn parse_enum() {
        let input = quote! {
            #[derive(Vertex)]
            enum Test {
                Unit,
                Unnammed(UnnamedParam1, UnnamedParam2),
                Named {
                    param1: NamedParam1,
                    param2: NamedParam2
                }
            }
        };
        let parse: DeriveInput = syn::parse2(input).expect("parse");
        let model = Model::try_from(DeriveType::Vertex(&parse)).expect("model");

        assert_debug_snapshot!(model);
    }
}
