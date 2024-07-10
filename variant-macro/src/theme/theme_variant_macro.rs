use proc_macro::TokenStream;
use std::collections::HashMap;

use darling::FromDeriveInput;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, LitStr};

use crate::theme::construct_theme_container;
use crate::theme::model::{TwTheme, TwThemeVariantContainer};

pub fn theme_variant_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let container = match TwThemeVariantContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let enum_ident = &container.ident;

    let variants = container.data.take_enum().unwrap_or_else(Vec::new);

    let base_theme_container = construct_theme_container(container.theme, enum_ident.to_string());

    let defaults = variants
        .iter()
        .filter(|v| v.default.is_present())
        .collect::<Vec<_>>();

    if defaults.is_empty() {
        return syn::Error::new(
            enum_ident.span(),
            "No default variant specified. Please mark one variant with `#[tw(default)]`",
        )
        .to_compile_error()
        .into();
    }

    if defaults.len() > 1 {
        let error = format!(
            "Only one variant can be marked as default: {:?}",
            defaults
                .iter()
                .map(|v| v.ident.to_string())
                .collect::<Vec<_>>()
        );
        let span = defaults[1].default.span();
        return syn::Error::new(span, error).to_compile_error().into();
    }

    let default_variant = defaults.into_iter().next().map(|v| {
        let variant_ident = &v.ident;
        quote! {
            impl Default for #enum_ident {
                fn default() -> Self {
                    #enum_ident::#variant_ident
                }
            }
        }
    });

    // Make a constant for each field and the base class
    let enum_ident_string = enum_ident.to_string().to_ascii_uppercase();
    let constants = variants
        .into_iter()
        .map(|variant| {
            let ident = variant.ident;
            let theme = variant.theme;
            let current_theme_dictionary = construct_theme_dictionary(
                theme,
                ident.to_string().to_ascii_uppercase(),
                enum_ident_string.clone(),
            );
            (ident, current_theme_dictionary)
        })
        .collect::<Vec<_>>();

    let to_class_cases = constants.iter().map(|(ident, theme_dict)| {
        let to_theme_class_cases = theme_dict.iter().map(|(name, (_class, ident))| {
            quote! {
                #name => #ident,
            }
        });

        let match_theme_class_cases = quote! {
            match theme.as_ref() {
                #( #to_theme_class_cases )*
                _ => "",
            }
        };

        quote! {
            #enum_ident::#ident =>  #match_theme_class_cases,
        }
    });

    let into_tailwind = quote! {
        impl AsTailwindThemeClass for #enum_ident {
            fn as_class(&self, theme: impl AsRef<str>) -> &str {
                match self {
                    #( #to_class_cases )*
                }
            }
        }
    };

    let constant_variables = constants.iter().flat_map(|(_variant, theme_dict)| {
        theme_dict.iter().map(|(name, (class, ident))| {
            if let Some((base_class, _)) = base_theme_container.get(name) {
                quote! {
                    const #ident: &'static str = concat!(#base_class, " ", #class);
                }
            } else {
                quote! {
                    const #ident: &'static str = #class;
                }
            }
        })
    });

    let gen = quote! {
        #default_variant

        #into_tailwind

        #( #constant_variables )*

        impl Copy for #enum_ident {}
        impl Clone for #enum_ident {
            fn clone(&self) -> Self {
                *self
            }
        }
    };

    gen.into()
}

fn construct_theme_dictionary<T>(
    iterator: T,
    ident: impl AsRef<str>,
    enum_ident: impl AsRef<str>,
) -> HashMap<LitStr, (String, Ident)>
where
    T: IntoIterator<Item = TwTheme>,
{
    iterator.into_iter().fold(
        HashMap::<LitStr, (String, Ident)>::new(),
        |mut container, item| {
            let value = item.class.as_ref().map(LitStr::value).unwrap_or_default();
            let key = item.name.unwrap_or(LitStr::new("base", Span::call_site()));
            let ident = Ident::new(
                &format!(
                    "{}_{}_{}",
                    key.value().to_ascii_uppercase(),
                    ident.as_ref().to_ascii_uppercase(),
                    enum_ident.as_ref().to_ascii_uppercase()
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
