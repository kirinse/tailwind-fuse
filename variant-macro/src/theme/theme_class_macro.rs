use proc_macro::TokenStream;

use darling::FromDeriveInput;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::theme::{construct_theme_container, TwThemeClassContainer};
use crate::theme::model::TwThemeClassField;

pub fn theme_class_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let container = match TwThemeClassContainer::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_ident = &container.ident;

    let builder_ident = format_ident!("{struct_ident}Builder");

    let fields = container
        .data
        .take_struct()
        .expect("Expected struct fields");

    let theme_container = construct_theme_container(container.theme, struct_ident.to_string());

    let constant_variables = theme_container.iter().map(|(_name, (class, ident))| {
        quote! {
            const #ident: &'static str = #class;
        }
    });

    let to_class_cases = theme_container.iter().map(|(name, (_class, ident))| {
        quote! {
            #name => #ident,
        }
    });

    let merger = {
        if let Some(merger) = container.merger {
            let ident = merger.as_ident();
            quote! {#ident}
        } else {
            quote! {TailwindMerge}
        }
    };

    let builder_struct = {
        let builder_fields = fields.iter().map(|field| {
            let TwThemeClassField { ident, ty, .. } = field;
            quote! { #ident: Option<#ty> }
        });
        quote! {
            #[derive(Copy, Clone, Default)]
            pub struct #builder_ident {
                #(#builder_fields,)*
            }
        }
    };

    let field_idents = fields
        .iter()
        .map(|field| field.ident.as_ref().expect("struct field has ident"))
        .collect::<Vec<_>>();

    let builder_impl = {
        let builder_set_methods = fields.iter().map(|field| {
            let TwThemeClassField { ident, ty, .. } = field;
            quote! {
                pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = Some(value);
                    self
                }
            }
        });

        quote! {
            impl #builder_ident {
                #(#builder_set_methods)*

                pub fn build(self) -> #struct_ident {
                    #struct_ident {
                        #(#field_idents: self.#field_idents.unwrap_or_default(),)*
                    }
                }
            }
        }
    };

    let builder_to_tailwind = {
        quote! {
            impl IntoTailwindThemeClass for #builder_ident {
                fn to_class(&self, theme: Option<impl AsRef<str>>) -> String {
                    self.with_class(theme, "")
                }

                fn with_class(&self, theme: Option<impl AsRef<str>>, class: impl AsRef<str>) -> String {
                    (*self).build().with_class(theme, class)
                }
            }
        }
    };

    let struct_to_tailwind = {
        let field_refs = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.as_class(name),
            }
        });

        quote! {
            impl IntoTailwindThemeClass for #struct_ident {
                fn to_class(&self, theme: Option<impl AsRef<str>>) -> String {
                    self.with_class(theme, "")
                }

                fn with_class(&self, theme: Option<impl AsRef<str>>, class: impl AsRef<str>) -> String {
                    let name = match theme {
                        Some(ref theme) => theme.as_ref(),
                        None => "base",
                    };
                    let theme_class = match name.as_ref() {
                        #( #to_class_cases )*
                        _ => "",
                    };
                    let classes = [
                        theme_class,
                        #( #field_refs )*
                        class.as_ref(),
                    ];
                    #merger.fuse_classes(&classes)
                }
            }
        }
    };

    let gen = quote! {
        #builder_struct

        #builder_impl

        #( #constant_variables )*

        impl IntoBuilder for #struct_ident {
            type Builder = #builder_ident;

            fn builder() -> Self::Builder {
                Default::default()
            }
            fn into_builder(self) -> Self::Builder {
                self.into()
            }
        }

        impl From<#struct_ident> for #builder_ident {
            fn from(value: #struct_ident) -> Self {
                #builder_ident {
                    #(#field_idents: Some(value.#field_idents),)*
                }
            }
        }

        impl From<#builder_ident> for #struct_ident {
            fn from(value: #builder_ident) -> Self {
                value.build()
            }
        }

        #builder_to_tailwind

        #struct_to_tailwind
    };

    gen.into()
}
