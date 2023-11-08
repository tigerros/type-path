//! # Guide
//!
//! The crate has just one macro: [`type_path!`](type_path).
//! Check it out for syntax and details about what it returns.
//!
//! You might be wondering why we're using an absolute path in the following example:
//!
//! ```rust
//! use type_path::type_path;
//! assert_eq!(type_path!(::std::io::BufWriter), ["::", "std", "io", "BufWriter"]);
//! ```
//!
//! It's because macros have no way of knowing the scope of the given item.
//!
//! If it wasn't required, the following would happen, because macros only have access to the literal `Result` tokens,
//! and no type information/scope etc.
//!
//! ```rust,ignore
//! # use type_path::type_path;
//! use std::io::Result;
//! assert_eq!(type_path!(Result), ["Result"]);
//! ```
//!
//! Now, the reliability aspect is gone, because even though the type exists,
//! `"Result"` is extremely generic and almost certainly not what you want.
//!
//! You can also have an "everything" path (although you probably won't use this a lot):
//!
//! ```rust
//! # use type_path::type_path;
//! assert_eq!(type_path!(::std::io::*), ["::", "std", "io", "*"]);
//! ```
//!
//! This doesn't compile, since `BufWrirer` doesn't exist:
//!
//! ```compile_fail,E0432
//! # use type_path::type_path;
//! type_path!(::std::io::BufWrirer);
//! ```
//!
//! # Why?
//! - **Reliability**:
//!   The [`type_path!`](type_path) macro will check if the path you entered is valid, at compile time.
//!   With other approaches, you can't be sure that the path is valid.
//! - No dependencies.
//! - You can use it in a no-std environment,

extern crate proc_macro;

use proc_macro::TokenStream;
use std::fmt::{Display, Formatter};
use proc_macro2::{Ident, Span};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};
use syn::punctuated::Punctuated;

enum AbsolutePathPrefix {
    DoubleColon(Token![::]),
    Crate((Token![crate], Token![::])),
}

impl Parse for AbsolutePathPrefix {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![::]) {
            Ok(AbsolutePathPrefix::DoubleColon(input.parse()?))
        } else if lookahead.peek(Token![crate]) {
            Ok(AbsolutePathPrefix::Crate((input.parse()?, input.parse()?)))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for AbsolutePathPrefix {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AbsolutePathPrefix::DoubleColon(_) => tokens.append_all(quote!(::)),
            AbsolutePathPrefix::Crate(_) => tokens.append_all(quote!(crate::)),
        }
    }
}

impl Display for AbsolutePathPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsolutePathPrefix::DoubleColon(_) => write!(f, "::"),
            AbsolutePathPrefix::Crate(_) => write!(f, "crate"),
        }
    }
}

mod kw {
    use syn::custom_punctuation;
    custom_punctuation!(ImportAll, ::*);
}

struct AbsolutePathMaybeImportAll {
    pub prefix: AbsolutePathPrefix,
    pub import_all: Option<kw::ImportAll>,
    pub segments: Punctuated<Ident, Token![::]>,
    /// Includes the prefix.
    pub string_segments: Vec<String>,
}

impl Parse for AbsolutePathMaybeImportAll {
    /// Unlike [`syn::Path`], this will not error if the tokens after a `::` are not [`Ident`]s.
    ///
    /// Trailing `::` is not allowed.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix: AbsolutePathPrefix = input.parse()?;
        let mut segments: Punctuated<Ident, Token![::]> = Punctuated::new();

        segments.push(input.parse()?);

        // Using peek3 here is necessary, because even though we only care about `::`,
        // `:` is a token and thus peek2 would just look at the second `:` in `::`.
        while input.peek(Token![::]) && input.peek3(syn::Ident) {
            input.parse::<Token![::]>()?;
            segments.push(input.parse()?);
        }

        let import_all: Option<kw::ImportAll> = input.parse()?;

        // + 2 for the prefix and potential import all star
        let mut string_segments = Vec::with_capacity(2 + segments.len());

        string_segments.push(prefix.to_string());
        string_segments.extend(segments.iter().map(Ident::to_string));

        if import_all.is_some() {
            string_segments.push("*".to_string());
        }

        Ok(AbsolutePathMaybeImportAll {
            prefix,
            import_all,
            segments,
            string_segments,
        })
    }
}

impl ToTokens for AbsolutePathMaybeImportAll {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let prefix = &self.prefix;
        let segments = self.segments.iter();

        tokens.append_all(quote!(#prefix #(#segments)::* ));

        if let Some(import_all) = self.import_all {
            tokens.append_all(quote!(#import_all));
        }
    }
}

/// # Syntax
///
/// There's 2 patterns:
///
/// - `::` prefix, with 1+ [identifiers](https://doc.rust-lang.org/reference/identifiers.html)
/// separated by `::`, with an optional `*` at the end.
/// E.g., `::path::to::Item`, `::path:to::prelude::*`.
/// This is an absolute path and will be resolved from the root of your project, regardless of scope.
/// - Same as the first pattern, but with a `crate` prefix.
/// Use this when you need to use types from the current crate.
///
/// # Returns
///
/// `[&'static str; N]`, where `N` is the amount of segments in the path, and `&str` is the segment.
///
/// The first item in the string array will be `"::"` or `"crate"`, depending on which prefix you use.
/// The last item will be `"*"` if the last segment in your path is `*`.
#[proc_macro]
pub fn type_path(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as AbsolutePathMaybeImportAll);
    let string_segments = &path.string_segments;

    let expanded = quote! {{
        #[allow(unused_imports)]
        use #path;

        [#(#string_segments),*]
    }};

    expanded.into_token_stream().into()
}

// TODO: Maybe create a file `paths.rs` and enforce proper structure. Would be finnicky, but might be worth it.
/// **Use [`type_path`] whenever you can instead of this.**
///
/// Use this only when the path you're getting is not accessible for whatever reason.
/// For example, in rustc development, re-exports will resolve to a private path.
///
/// **See the [returns section](#returns) before proceeding.**
///
/// # Syntax
///
/// Same as [`type_path`].
///
/// # Returns
///
/// A constant representing the path.
/// The reason is to force your code to be consistent.
///
/// The constant always has the `pub(crate)` visibility.
/// You should not be exporting this constant.
/// TODO explain why you can't restrict it further. That might be obsolote though, see regular comment TODO above.
///
/// The constant's name is: `PATH_{PREFIX}_{PATH_SEGMENTS}` where:
///
/// - `PATH_`: Always present.
/// - `{PREFIX}`: `CC` if your path starts with `::` or `CRATE` if it starts with `crate::`.
/// - `{PATH_SEGMENTS}`: The path segments after the prefix, joined with `_cc_` (`::`; double **c**olon),
/// and turned uppercase.
/// `UpperCamelCase` names, such as structs, will be turned into `SCREAMING_SNAKE_CASE`.
///
/// The `_cc_` separator looks weird,
/// but it ensures that the constant name can always be traced back a type path.
/// If it was uppercase, it wouldn't be guaranteed, since there could be some path segment called `cc`
/// which would turns into `_CC_`.
///
/// See the [examples section](#examples) to see this in action.
///
/// # Examples
///
/// Some foreign crate types:
///
/// ```rust
/// // foreign-crate/lib.rs
///
/// mod private_mod {
///     pub struct ExportedStruct;
/// }
///
/// // Rustc will resolve this to the private path (`foreign_crate::private_mod::ExportedStruct`).
/// // Therefore, tools like Clippy need to check for the private path.
/// pub use private_mod::ExportedStruct;
/// ```
///
/// Your crate which uses [`inaccessible_type_path`]:
///
/// ```rust
/// // your-crate/paths.rs
///
/// use type_path::inaccessible_type_path;
///
/// inaccessible_type_path!(::foreign_crate::private_mod::ExportedStruct);
/// ```
///
/// The above snippet will produce:
///
/// ```rust
/// // your-crate/paths.rs
///
/// #[allow(non_upper_case_globals)]
/// pub(crate) const PATH_cc_FOREIGN_CRATE_cc_EXPORTED_STRUCT: [&str; 4] = ["::", "foreign_crate", "private_mod", "ExportedStruct"];
/// ```
///
/// See the [returns section](#returns) to see why the constant name looks weird or other concerns.
#[proc_macro]
pub fn inaccessible_type_path(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as AbsolutePathMaybeImportAll);
    let string_segments = &path.string_segments;
    let segment_count = string_segments.len();

    let path_const_name = string_segments.iter().fold("PATH".to_string(), |mut curr, next| {
        curr.push('_');
        curr.push_str(&next.to_uppercase());
        curr
    });

    let path_const_ident = Ident::new(&path_const_name, Span::call_site());

    let expanded = quote! {
        pub(crate) const #path_const_ident : [&str; #segment_count] = [#(#string_segments),*];
    };

    expanded.into_token_stream().into()
}