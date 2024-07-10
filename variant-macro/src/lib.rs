use proc_macro::TokenStream;

use class_macro::class_impl;
use variant_macro::variant_impl;

use crate::theme::{theme_class_impl, theme_variant_impl};

mod class_macro;
mod model;
mod theme;
mod variant_macro;

#[proc_macro_derive(TwVariant, attributes(tw))]
pub fn variant(input: TokenStream) -> TokenStream {
    variant_impl(input)
}

#[proc_macro_derive(TwClass, attributes(tw))]
pub fn class(input: TokenStream) -> TokenStream {
    class_impl(input)
}

#[proc_macro_derive(TwThemeClass, attributes(tw))]
pub fn theme_class(input: TokenStream) -> TokenStream {
    theme_class_impl(input)
}

#[proc_macro_derive(TwThemeVariant, attributes(tw))]
pub fn theme_variant(input: TokenStream) -> TokenStream {
    theme_variant_impl(input)
}
