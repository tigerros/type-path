#![allow(non_ascii_idents)]
#![allow(uncommon_codepoints)]

use type_path::inaccessible_type_path;
use type_path::type_path;

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/**/*.rs");
}

#[test]
fn three_segment_path() {
    assert_eq!(
        type_path!(::std::io::BufWriter),
        ["::", "std", "io", "BufWriter"]
    );
}

#[test]
fn one_segment_path() {
    assert_eq!(type_path!(::core), ["::", "core"]);
}

#[test]
fn import_everything() {
    assert_eq!(type_path!(::core::*), ["::", "core", "*"]);
}

#[allow(unused)]
mod foo {
    pub mod bar {
        pub const BAZ: bool = false;
        pub const FOZ: bool = false;
    }

    mod private_mod {}
}

#[test]
fn private_path() {
    inaccessible_type_path!(crate::foo::private_mod);

    assert_eq!(PATH_CRATE_FOO_PRIVATE_MOD, ["crate", "foo", "private_mod"]);
}

#[test]
fn crate_prefix() {
    assert_eq!(type_path!(crate::foo::bar), ["crate", "foo", "bar"]);
}

#[test]
fn import_everything_with_crate_prefix() {
    assert_eq!(type_path!(crate::foo::bar::*), ["crate", "foo", "bar", "*"]);
}

#[test]
#[rustfmt::skip]
fn extra_whitespace() {
    const TARGET: [&str; 4] = ["::", "std", "path", "PathBuf"];

    assert_eq!(type_path!(::    std::     path   ::PathBuf), TARGET);
    assert_eq!(type_path!( :: std::     path::      PathBuf), TARGET);
    assert_eq!(type_path!(
::
        std::path::PathBuf), TARGET);
    assert_eq!(type_path!(
  ::

                std
        ::
        path::
        PathBuf

    ), TARGET);
}

#[allow(dead_code)]
mod 例 {
    pub const 傅: bool = false;
    pub const 𐊜: bool = false;
}

#[test]
#[rustfmt::skip]
fn non_ascii_chars() {
    // Whitespace to test trimming
    assert_eq!(type_path!(

        crate::      例::

        傅), ["crate", "例", "傅"]);

    assert_eq!(type_path!(

        crate::      例::

        𐊜), ["crate", "例", "𐊜"]);
}
