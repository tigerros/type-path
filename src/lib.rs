#![no_std]

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

const WHITESPACE: u8 = b'\t' | b'\n' | b'\r' | b' ';

const fn bytes_trim_start(mut this: &[u8]) -> &[u8] {
    loop {
        match this {
            [WHITESPACE, rem @ ..] => this = rem,
            _ => return this,
        }
    }
}

const fn bytes_trim_end(mut this: &[u8]) -> &[u8] {
    loop {
        match this {
            [rem @ .., WHITESPACE] => this = rem,
            _ => return this,
        }
    }
}

const fn bytes_trim(this: &[u8]) -> &[u8] {
    bytes_trim_start(bytes_trim_end(this))
}

/// **This is not intended to be used outside of this crate!**
///
/// Trims a string at compile time.
#[doc(hidden)]
#[must_use]
pub const fn trim(this: &str) -> &str {
    let trimmed = bytes_trim(this.as_bytes());

    // Safety: bytes_trim only removes ascii bytes
    unsafe { core::str::from_utf8_unchecked(trimmed) }
}

/// **This is not intended to be used outside of this crate!**
///
/// Macro with one empty pattern `() => {};`.
/// Used for enforcing that a meta variable (e.g., `$foo:tt`) is empty.
#[macro_export]
#[doc(hidden)]
macro_rules! empty {
    () => {};
}

/// # Syntax
///
/// There's 2 patterns:
///
/// - `::` prefix, with 1+ [identifiers] separated by `::`, with an optional `*` at the end.
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
///
/// [identifiers]: https://doc.rust-lang.org/reference/identifiers.html "Rust identifier reference"
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! type_path {

    (::$($segment:ident):: +$(::*$($empty:tt)? )?) => {{
        #[allow(unused_imports)]
        use :: $($segment)::+ $(::* $($crate::empty!($empty))?)?;

        ["::", $($crate::trim(stringify!($segment))),+ $(, "*" $($crate::empty!($empty))?)?]
    }};

    (crate::$($segment:ident):: +$(::*$($empty:tt)? )?) => {{
        #[allow(unused_imports)]
        use crate :: $($segment)::+ $(::* $($crate::empty!($empty))?)?;

        ["crate", $($crate::trim(stringify!($segment))),+ $(, "*" $($crate::empty!($empty))?)?]
    }};
}
