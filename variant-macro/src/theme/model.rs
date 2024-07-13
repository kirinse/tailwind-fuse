use darling::{ast, FromDeriveInput, FromField, FromMeta, FromVariant};
use darling::util::{Flag, IdentString};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tw), supports(enum_unit))]
pub struct TwThemeVariantContainer {
    pub ident: syn::Ident,
    pub data: ast::Data<TwThemeVariantOption, ()>,
    #[darling(multiple)]
    pub theme: Vec<TwTheme>,
}

#[derive(Debug, FromVariant)]
#[darling(supports(unit), attributes(tw, default))]
pub struct TwThemeVariantOption {
    pub ident: syn::Ident,
    #[darling(multiple)]
    pub theme: Vec<TwTheme>,
    pub default: Flag,
}

#[derive(Default, Debug, FromMeta)]
pub struct TwTheme {
    /// If no name is provided or the value of name is specified as base,
    /// then this class means that all subject items are shared.
    pub name: Option<syn::Ident>,
    pub class: Option<syn::LitStr>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tw), supports(struct_named))]
pub struct TwThemeClassContainer {
    pub ident: syn::Ident,
    pub data: ast::Data<(), TwThemeClassField>,
    #[darling(multiple)]
    pub theme: Vec<TwTheme>,
    /// Defaults to using `tw_merge`.
    pub merger: Option<IdentString>,
}

#[derive(Debug, FromField)]
#[darling(attributes(tw))]
pub struct TwThemeClassField {
    pub ty: syn::Type,
    pub ident: Option<syn::Ident>,
}
