[![tests badge]](https://github.com/tigerros/type-path/actions/workflows/tests.yml)
[![lints badge]](https://github.com/tigerros/type-path/actions/workflows/lints.yml)
[![docs.rs badge]](https://docs.rs/type-path/)
[![crates.io badge]](https://crates.io/crates/type-path)

A tiny crate for getting the string array representation of a Rust type path, with type validation.
Everything happens at compile-time.

Check out the docs for everything else!

[tests badge]: https://img.shields.io/github/actions/workflow/status/tigerros/type-path/tests.yml?label=tests&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48IS0tISBGb250IEF3ZXNvbWUgUHJvIDYuNC4yIGJ5IEBmb250YXdlc29tZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tIExpY2Vuc2UgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbS9saWNlbnNlIChDb21tZXJjaWFsIExpY2Vuc2UpIENvcHlyaWdodCAyMDIzIEZvbnRpY29ucywgSW5jLiAtLT48cGF0aCBmaWxsPSIjRkZGRkZGIiBkPSJNMzQyLjYgOS40Yy0xMi41LTEyLjUtMzIuOC0xMi41LTQ1LjMgMHMtMTIuNSAzMi44IDAgNDUuM2w5LjQgOS40TDI4LjEgMzQyLjZDMTAuMSAzNjAuNiAwIDM4NSAwIDQxMC41VjQxNmMwIDUzIDQzIDk2IDk2IDk2aDUuNWMyNS41IDAgNDkuOS0xMC4xIDY3LjktMjguMUw0NDggMjA1LjNsOS40IDkuNGMxMi41IDEyLjUgMzIuOCAxMi41IDQ1LjMgMHMxMi41LTMyLjggMC00NS4zbC0zMi0zMi05Ni05Ni0zMi0zMnpNMjA1LjMgMjU2TDM1MiAxMDkuMyA0MDIuNyAxNjBsLTk2IDk2SDIwNS4zeiIvPjwvc3ZnPg==
[lints badge]: https://img.shields.io/github/actions/workflow/status/tigerros/type-path/lints.yml?label=lints&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1NzYgNTEyIj48IS0tISBGb250IEF3ZXNvbWUgUHJvIDYuNC4yIGJ5IEBmb250YXdlc29tZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tIExpY2Vuc2UgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbS9saWNlbnNlIChDb21tZXJjaWFsIExpY2Vuc2UpIENvcHlyaWdodCAyMDIzIEZvbnRpY29ucywgSW5jLiAtLT48cGF0aCBmaWxsPSIjRkZGRkZGIiBkPSJNMTEyIDBDOTkuMSAwIDg3LjQgNy44IDgyLjUgMTkuN2wtNjYuNyAxNjAtMTMuMyAzMmMtNi44IDE2LjMgLjkgMzUgMTcuMiA0MS44czM1LS45IDQxLjgtMTcuMkw2Ni43IDIyNGg5MC43bDUuMSAxMi4zYzYuOCAxNi4zIDI1LjUgMjQgNDEuOCAxNy4yczI0LTI1LjUgMTcuMi00MS44bC0xMy4zLTMyLTY2LjctMTYwQzEzNi42IDcuOCAxMjQuOSAwIDExMiAwem0xOC43IDE2MEg5My4zTDExMiAxMTUuMiAxMzAuNyAxNjB6TTI1NiAzMnY5NiA5NmMwIDE3LjcgMTQuMyAzMiAzMiAzMmg4MGM0NC4yIDAgODAtMzUuOCA4MC04MGMwLTIzLjEtOS44LTQzLjgtMjUuNC01OC40YzYtMTEuMiA5LjQtMjQgOS40LTM3LjZjMC00NC4yLTM1LjgtODAtODAtODBIMjg4Yy0xNy43IDAtMzIgMTQuMy0zMiAzMnptOTYgNjRIMzIwVjY0aDMyYzguOCAwIDE2IDcuMiAxNiAxNnMtNy4yIDE2LTE2IDE2em0tMzIgNjRoMzIgMTZjOC44IDAgMTYgNy4yIDE2IDE2cy03LjIgMTYtMTYgMTZIMzIwVjE2MHpNNTY2LjYgMzEwLjZjMTIuNS0xMi41IDEyLjUtMzIuOCAwLTQ1LjNzLTMyLjgtMTIuNS00NS4zIDBMMzUyIDQzNC43bC03My40LTczLjRjLTEyLjUtMTIuNS0zMi44LTEyLjUtNDUuMyAwcy0xMi41IDMyLjggMCA0NS4zbDk2IDk2YzEyLjUgMTIuNSAzMi44IDEyLjUgNDUuMyAwbDE5Mi0xOTJ6Ii8+PC9zdmc+
[docs.rs badge]: https://img.shields.io/docsrs/type-path?logo=docs.rs&label=docs.rs
[crates.io badge]: https://img.shields.io/crates/v/type-path?logo=rust