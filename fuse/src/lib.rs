#![forbid(missing_docs)]

//! # Tailwind Fuse
//! [<img alt="github" src="https://img.shields.io/badge/github-gaucho--labs/tailwind--fuse-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/gaucho-labs/tailwind-fuse)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/tailwind-fuse.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tailwind-fuse)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tailwind--fuse-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tailwind-fuse)
//! [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gaucho-labs/tailwind-fuse/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/gaucho-labs/tailwind-fuse/actions?query=branch%3Amain)
//!
//! Two main utils are included in this crate:
//!
//! 1. Fuse: Fuse multiple Tailwind classes, with optional conflict resolution.
//!     > Inspired by [Tailwind Merge](https://github.com/dcastil/tailwind-merge)
//! 2. Variants: Compose type-safe variant classes
//!     > Inspired by [Class Variance Authority](https://github.com/joe-bell/cva)
//!
//!
//! ## Installation
//!
//! Variants require the `variant` feature to be enabled.
//!
//! #### With variant
//! ```bash
//! cargo add tailwind-fuse --features variant
//! ```
//!
//! #### Without variant
//! ```bash
//! cargo add tailwind-fuse
//! ```
//!
//! ## Usage: Fuse
//!
//! You can use [`tw_join!`] to join Tailwind classes, and [`tw_merge!`] to merge Tailwind Classes handling conflicts.
//!
//!
//! You can use anything that implements [`AsTailwindClass`]
//!
//! ```
//! use tailwind_fuse::*;
//!
//! // No conflict resolution
//! assert_eq!(
//!    "flex items-center justify-center",
//!    tw_join!("flex", "items-center", "justify-center")
//! );
//!
//! // Conflict resolution
//! // Right most class takes precedence
//! assert_eq!(
//!    "p-4",
//!    tw_merge!("py-2 px-4", "p-4")
//! );
//!
//! // Refinements are permitted
//! assert_eq!(
//!    "p-4 py-2",
//!    tw_merge!("p-4", "py-2")
//! );
//! ```
//!
//! You can use Options to exclude certain classes from being merged
//!
//! ```
//! use tailwind_fuse::*;
//!
//! assert_eq!(
//!   "flex justify-center",
//!   tw_join!("flex", (false).then_some("items-center"), (true).then_some("justify-center"))
//! )
//! ```
//!
//! ### Custom Tailwind Prefix/Separator
//!
//! Use [`merge::set_merge_options`] to set global options for [`tw_merge!`] and variant macros.
//!
//! This can only be set once. Subsequent calls will be ignored.
//!
//! ```
//! use tailwind_fuse::{*, merge::*};
//!
//! const OPTIONS: MergeOptions = MergeOptions {
//!     prefix: "tw-",
//!     separator: ":",
//! };
//!
//! // Before setting options, the default (no prefix) is used
//! assert_eq!(
//!   "tw-bg-black tw-bg-white",
//!   tw_merge!("tw-bg-black", "tw-bg-white"),
//! );
//!
//! set_merge_options(OPTIONS);
//!
//! assert_eq!(
//!   "tw-bg-white",
//!   tw_merge!("tw-bg-black", "tw-bg-white"),
//! );
//!
//! ```
//!
//!
//! ## Usage: Variants
//!
//! Useful for building components with first class support for tailwind. By default, conflicts are merged using [`tw_merge()`].
//!
//! Each [`TwClass`] represents a UI element with customizable properties (property is a "variant"). Each variant is represented by a [`TwVariant`], which must be an enum with a default case.
//!
//! The classes are merged in the following order, with the last class takes precedence:
//! 1. Base class from [`TwClass`]
//! 2. Base class from [`TwVariant`]
//! 3. Enum variant class from [`TwVariant`]
//! 4. Override class using [`IntoTailwindClass::with_class`] on the struct or builder
//!
//! ```
//! use tailwind_fuse::*;
//!
//! // Your Component Type
//! #[derive(TwClass)]
//! // Optional base class
//! #[tw(class = "flex")]
//! struct Btn {
//!     size: BtnSize,
//!     color: BtnColor,
//! }
//!
//! // Variant for size
//! #[derive(TwVariant)]
//! enum BtnSize {
//!     #[tw(default, class = "h-9 px-4 py-2")]
//!     Default,
//!     #[tw(class = "h-8 px-3")]
//!     Sm,
//!     #[tw(class = "h-10 px-8")]
//!     Lg,
//! }
//!
//! // Variant for color
//! #[derive(TwVariant)]
//! enum BtnColor {
//!     #[tw(default, class = "bg-blue-500 text-blue-100")]
//!     Blue,
//!     #[tw(class = "bg-red-500 text-red-100")]
//!     Red,
//! }
//! ```
//!
//! You can now use the `Btn` struct to generate Tailwind classes, using builder syntax, or using the struct directly
//!
//! ### Struct Syntax
//! ```
//! # use tailwind_fuse::*;
//! # // Your Component Type
//! # #[derive(TwClass)]
//! # // Optional base class
//! # #[tw(class = "flex")]
//! # struct Btn {
//! #     size: BtnSize,
//! #     color: BtnColor,
//! # }
//! # // Variant for size
//! # #[derive(TwVariant)]
//! # enum BtnSize {
//! #     #[tw(default, class = "h-9 px-4 py-2")]
//! #     Default,
//! #     #[tw(class = "h-8 px-3")]
//! #     Sm,
//! #     #[tw(class = "h-10 px-8")]
//! #     Lg,
//! # }
//! # // Variant for color
//! # #[derive(TwVariant)]
//! # enum BtnColor {
//! #     #[tw(default, class = "bg-blue-500 text-blue-100")]
//! #     Blue,
//! #     #[tw(class = "bg-red-500 text-red-100")]
//! #     Red,
//! # }
//! let button = Btn {
//!     size: BtnSize::Default,
//!     color: BtnColor::Blue,
//! };
//!
//! assert_eq!(
//!    "flex h-9 px-4 py-2 bg-blue-500 text-blue-100",
//!    button.to_class()
//! );
//!
//! // Conflicts are resolved (bg-blue-500 is knocked out in favor of override)
//! assert_eq!(
//!    "flex h-9 px-4 py-2 text-blue-100 bg-green-500",
//!    button.with_class("bg-green-500")
//! );
//! ```
//!
//! ### Builder Syntax
//! You access the builder using the `variants` method. Every variant that is not provided will be replaced with the default variant.
//!
//! ```
//! # use tailwind_fuse::*;
//! #
//! # #[derive(TwClass)]
//! # // Optional base class
//! # #[tw(class = "flex")]
//! # struct Btn {
//! #     size: BtnSize,
//! #     color: BtnColor,
//! # }
//! #
//! # // Variant for size
//! # #[derive(TwVariant)]
//! # enum BtnSize {
//! #     #[tw(default, class = "h-9 px-4 py-2")]
//! #     Default,
//! #     #[tw(class = "h-8 px-3")]
//! #     Sm,
//! #     #[tw(class = "h-10 px-8")]
//! #     Lg,
//! # }
//! #
//! # // Variant for color
//! # #[derive(TwVariant)]
//! # enum BtnColor {
//! #     #[tw(default, class = "bg-blue-500 text-blue-100")]
//! #     Blue,
//! #     #[tw(class = "bg-red-500 text-red-100")]
//! #     Red,
//! # }
//!
//! assert_eq!(
//!    "flex h-8 px-3 bg-red-500 text-red-100",
//!    Btn::builder()
//!       .size(BtnSize::Sm)
//!       .color(BtnColor::Red)
//!       .to_class()
//! );
//!
//! assert_eq!(
//!    "flex h-8 px-3 text-red-100 bg-green-500",
//!    Btn::builder()
//!       .size(BtnSize::Sm)
//!       .color(BtnColor::Red)
//!       .with_class("bg-green-500")
//! );
//!
//! ```
//!
//! #### VSCode Intellisense
//!
//! You can enable autocompletion inside `#[tw()]` using the steps below:
//!
//! 1. [Install the "Tailwind CSS IntelliSense" Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
//!
//! 2. Add the following to your [`settings.json`](https://code.visualstudio.com/docs/getstarted/settings):
//!
//! ```json
//! {
//!   "tailwindCSS.experimental.classRegex": [
//!     ["#[tw\\\\([^\\]]*class\\s*=\\s*\"([^\"]*)\"\\)]", "\"([^\"]*)\""]
//!   ]
//! }
//! ```
//!

#[cfg(feature = "variant")]
pub use variant::*;

pub use crate::core::*;
pub use crate::core::merge;

mod ast;
mod core;

#[cfg(feature = "variant")]
mod variant {
    /// Derives a class for use with Tailwind CSS in Rust components.
    ///
    /// Allows building components with first-class support for Tailwind.
    ///
    /// Defaults to using [`crate::tw_merge()`] to resolve conflicts.
    ///
    /// Resolves conflicts using the following merge order:
    /// - [`TwClass`] base class
    /// - [`TwVariant`] base class
    /// - [`TwVariant`] enum variant class
    /// - Override class with `with_class`
    ///
    /// # Example
    ///
    /// ```rust
    /// use tailwind_fuse::*;
    ///
    /// #[derive(TwClass, Debug)]
    /// // Optional base class.
    /// #[tw(class = "flex")]
    /// struct Btn {
    ///     size: BtnSize,
    ///     color: BtnColor,
    /// }
    ///
    /// #[derive(TwVariant, Debug)]
    /// enum BtnSize {
    ///     #[tw(default, class = "h-9 px-4 py-2")]
    ///     Default,
    ///     #[tw(class = "h-8 px-3")]
    ///     Sm,
    ///     #[tw(class = "h-10 px-8")]
    ///     Lg,
    /// }
    ///
    /// #[derive(TwVariant, Debug)]
    /// enum BtnColor {
    ///     #[tw(default, class = "bg-blue-500 text-blue-100")]
    ///     Blue,
    ///     #[tw(class = "bg-red-500 text-red-100")]
    ///     Red,
    /// }
    ///
    /// let btn = Btn { size: BtnSize::Default, color: BtnColor::Blue };
    /// assert_eq!(btn.to_class(), "flex h-9 px-4 py-2 bg-blue-500 text-blue-100");
    ///
    /// let btn_variant = Btn::builder().color(BtnColor::Red).to_class();
    /// assert_eq!(btn_variant, "flex h-9 px-4 py-2 bg-red-500 text-red-100");
    /// ```
    pub use tailwind_fuse_macro::TwClass;
    /// Derives a class for use with Tailwind CSS in Rust components.
    ///
    /// Allows building components with first-class support for Tailwind.
    ///
    /// Defaults to using [`crate::tw_merge()`] to resolve conflicts.
    ///
    /// Resolves conflicts using the following merge order:
    /// - [`TwThemeClass`] base class
    /// - Override class with `with_class`
    ///
    /// # Example
    ///
    /// ```rust
    /// use tailwind_fuse::*;
    ///
    /// #[derive(TwThemeClass)]
    /// #[tw(theme(class = "flex"))]
    /// #[tw(theme(name = "default", class = "flex-col items-center"))]
    /// struct Btn {
    ///     btn_size: BtnSize,
    /// }
    ///
    /// #[derive(TwThemeVariant)]
    /// enum BtnSize {
    ///     #[tw(default)]
    ///     #[tw(theme(class = "h-9"))]
    ///     #[tw(theme(name = "default", class = "px-4"))]
    ///     Default,
    ///     #[tw(theme(class = "h-8"))]
    ///     #[tw(theme(name = "default", class = "text-xs"))]
    ///     Sm,
    ///     #[tw(theme(class = "h-10"))]
    ///     #[tw(theme(name = "default", class = "rounded-lg"))]
    ///     Lg,
    /// }
    ///
    /// let btn = BtnBuilder::default().btn_size(BtnSize::Lg).build();
    /// assert_eq!("flex-col items-center rounded-lg", btn.to_class(Some("default")));
    /// assert_eq!("flex h-10", btn.to_class(None::<&str>))
    /// ```
    pub use tailwind_fuse_macro::TwThemeClass;
    /// Represents a customizable property (variant) of a UI element.
    /// Each variant must be an enum with a default case.
    ///
    /// Use `.to_class()` to get the class for the variant and `.with_class()` to append a class.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tailwind_fuse::*;
    ///
    /// #[derive(TwThemeVariant)]
    /// #[tw(theme(class = "text-gray-100"))]
    /// #[tw(theme(name = "default", class = "text-red-500"))]
    /// enum BtnSize {
    ///     #[tw(default)]
    ///     #[tw(theme(class = "h-9"))]
    ///     #[tw(theme(name = "default", class = "px-4"))]
    ///     Default,
    ///     #[tw(theme(class = "h-8"))]
    ///     #[tw(theme(name = "default", class = "text-xs"))]
    ///     Sm,
    ///     #[tw(theme(class = "h-10"))]
    ///     #[tw(theme(name = "default", class = "rounded-lg"))]
    ///     Lg,
    /// }
    ///
    /// assert_eq!("text-gray-100 h-9", BtnSize::Default.as_class("base"));
    /// assert_eq!("text-red-500 px-4", BtnSize::Default.as_class("default"));
    /// ```
    pub use tailwind_fuse_macro::TwThemeVariant;
    /// Represents a customizable property (variant) of a UI element.
    /// Each variant must be an enum with a default case.
    ///
    /// Use `.to_class()` to get the class for the variant and `.with_class()` to append a class.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tailwind_fuse::*;
    ///
    /// #[derive(TwVariant, Debug)]
    /// // Optional base class
    /// #[tw(class = "hover:brightness-50")]
    /// enum BtnColor {
    ///     #[tw(default, class = "bg-blue-500 text-blue-100")]
    ///     Default,
    ///     #[tw(class = "bg-red-500 text-red-100")]
    ///     Red,
    /// }
    ///
    /// assert_eq!("hover:brightness-50 bg-blue-500 text-blue-100", BtnColor::Default.as_class());
    /// assert_eq!("hover:brightness-50 bg-red-500 text-red-100", BtnColor::Red.as_class());
    /// ```
    pub use tailwind_fuse_macro::TwVariant;

    /// Used to Fuse Tailwind Classes together.
    pub trait TailwindFuse {
        /// Strings are not guaranteed to be single class nor free of whitespace.
        fn fuse_classes(&self, class: &[&str]) -> String;
    }

    /// Will merge Tailwind classes and handle conflicts using [`crate::merge::tw_merge_slice`]
    pub struct TailwindMerge;

    impl TailwindFuse for TailwindMerge {
        fn fuse_classes(&self, class: &[&str]) -> String {
            crate::merge::tw_merge_slice(class)
        }
    }

    /// Will simply join Tailwind classes together without handling conflicts
    pub struct TailwindJoin;

    impl TailwindFuse for TailwindJoin {
        fn fuse_classes(&self, class: &[&str]) -> String {
            class
                .iter()
                .flat_map(|s| s.split_whitespace())
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .fold(String::new(), |mut acc, s| {
                    if !acc.is_empty() {
                        acc.push(' ');
                    }
                    acc.push_str(s);
                    acc
                })
        }
    }

    /// A trait to convert a type into a Tailwind class.
    /// Implemented automatically for usages of [`TwClass`] and [`TwVariant`].
    pub trait IntoTailwindClass {
        /// Convert the type into a Tailwind class.
        fn to_class(&self) -> String;
        /// Append to the class (with override precedence) and return the new class.
        fn with_class(&self, class: impl AsRef<str>) -> String;
    }

    /// Used to extract a &str from a type
    pub trait AsTailwindThemeClass {
        /// Extract a Tailwind class
        fn as_class(&self, theme: impl AsRef<str>) -> &str;
    }

    /// A trait to convert a type into a Tailwind class.
    /// Implemented automatically for usages of [`TwThemeClass`] and [`TwThemeVariant`].
    pub trait IntoTailwindThemeClass {
        /// Convert the type into a Tailwind class.
        fn to_class(&self, theme: Option<impl AsRef<str>>) -> String;
        /// Append to the class (with override precedence) and return the new class.
        fn with_class(&self, theme: Option<impl AsRef<str>>, class: impl AsRef<str>) -> String;
    }

    /// Converts a type into it's builder.
    /// Automatically implemented for usages of [`TwClass`].
    pub trait IntoBuilder {
        /// The builder type.
        type Builder;
        /// Get a builder instance
        fn builder() -> Self::Builder;
        /// Convert the instance into the builder.
        fn into_builder(self) -> Self::Builder;
    }
}
