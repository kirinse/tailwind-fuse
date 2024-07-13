use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use syn::LitStr;

use model::TwTheme;
use model::TwThemeClassContainer;
pub use theme_class_macro::theme_class_impl;
pub use theme_variant_macro::theme_variant_impl;

mod model;
mod theme_class_macro;
mod theme_variant_macro;

fn construct_theme_container<T>(
    iterator: T,
    ident: impl AsRef<str>,
) -> HashMap<LitStr, (String, Ident)>
where
    T: IntoIterator<Item = TwTheme>,
{
    iterator.into_iter().fold(
        HashMap::<LitStr, (String, Ident)>::new(),
        |mut container, item| {
            let value = item.class.as_ref().map(LitStr::value).unwrap_or_default();
            let key = match item.name {
                Some(name) => LitStr::new(&name.to_string(), name.span()),
                None => LitStr::new("base", Span::call_site()),
            };
            let ident = Ident::new(
                &format!(
                    "{}_{}",
                    key.value().to_ascii_uppercase(),
                    ident.as_ref().to_ascii_uppercase()
                ),
                Span::call_site(),
            );
            let (ref mut class, _) = container.entry(key).or_insert(("".to_owned(), ident));
            if !class.is_empty() {
                class.push(' ');
            }
            class.push_str(value.as_str());
            container
        },
    )
}
