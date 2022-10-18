//! [<img alt="github" src="https://img.shields.io/badge/github-udoprog/hodoku-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/hodoku)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/hodoku.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/hodoku)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-hodoku-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/hodoku)
//! [<img alt="build status" src="https://img.shields.io/github/workflow/status/udoprog/hodoku/CI/main?style=for-the-badge" height="20">](https://github.com/udoprog/hodoku/actions?query=branch%3Amain)
//!
//! A simple set of macros to aid testing with try operations.
//!
//! This crate allows for easily writing functions and expression where `?` is
//! automatically translated into `.unwrap()`.
//!
//! It is syntactically desirable to use `?`. This however causes issues during
//! testing, because a failing test lacks a stack trace which helps you track
//! down the exact line that errored.
//!
//! ```
//! # fn function() -> Result<u32, &'static str> { Ok(42) };
//! #[test]
//! fn test_case() -> Result<(), &'static str> {
//!     let value = function()?;
//!     assert_eq!(value, 42);
//!     Ok(())
//! }
//! ```
//!
//! By default you'd get this when `function()?` errors:
//!
//! ```text
//! ---- test_case stdout ----
//! Error: "bad"
//! thread 'test_case' panicked at 'assertion failed: `(left == right)`
//!   left: `1`,
//!  right: `0`: the test returned a termination value with a non-zero status code (1) which indicates a failure', <path>\library\test\src\lib.rs:185:5
//! note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//!
//!
//! failures:
//!     test_case
//! ```
//!
//! Note how there's no information on which line the test failed on.
//!
//! But with the inclusion of `#[hodoku::function]` you get this:
//!
//! ```
//! # fn function() -> Result<u32, &'static str> { Err("bad") };
//! #[test]
//! #[hodoku::function]
//! fn test_case() -> Result<(), &'static str> {
//!     let value = function()?;
//!     assert_eq!(value, 42);
//!     Ok(())
//! }
//! ```
//!
//! ```text
//! ---- test_case stdout ----
//! thread 'test_case' panicked at 'called `Result::unwrap()` on an `Err` value: "bad"', tests\failing.rs:8:27
//! note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//!
//!
//! failures:
//!     test_case
//! ```
//!
//! This is exactly why we want to make use of `.unwrap()` instead of the try
//! operator tests. It indicates the exact line that errored.
//!
//! <br>
//!
//! ## Examples
//!
//! Use of `#[hodoku::function]`.
//!
//! ```
//! #[hodoku::function]
//! fn hello() {
//!     let value = Some(42)?;
//!     assert_eq!(value, 42);
//! }
//!
//! hello();
//! ```
//!
//! Unwrapping expressions:
//!
//! ```
//! let value = hodoku::expr!(Some(42)?);
//! assert_eq!(value, 42);
//! ```

use std::array;
use std::iter;

use proc_macro::Spacing;
use proc_macro::{Delimiter, Group, Ident, Punct, TokenStream, TokenTree};

/// Process an expression or item marked with an attribute to modify any uses of
/// the try operator `?` into trailing `.unwrap()`. So `Some(42)?` will be
/// translated to `Some(42).unwrap()`.
///
/// This is useful for adhoc testing.
///
/// # Examples
///
/// ```
/// #[hodoku::function]
/// fn hello() {
///     let value = Some(42)?;
///     assert_eq!(value, 42);
/// }
///
/// hello();
/// ```
#[proc_macro_attribute]
pub fn function(args: TokenStream, item: TokenStream) -> TokenStream {
    if let Some(..) = args.into_iter().next() {
        panic!("#[hodoku::function]: takes not arguments")
    }

    process(item)
}

/// Process an expression to modify any uses of the try operator `?` into
/// trailing `.unwrap()`. So `expr!(Some(42)?)` will be translated to
/// `Some(42).unwrap()`.
///
/// This is useful for adhoc testing.
///
/// # Examples
///
/// ```
/// let value = hodoku::expr!(Some(42)?);
/// assert_eq!(value, 42);
/// ```
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    process(input)
}

fn process(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    let mut tmp = None::<array::IntoIter<TokenTree, 2>>;

    TokenStream::from_iter(iter::from_fn(move || {
        if let Some(buf) = tmp.as_mut() {
            if let Some(tt) = buf.next() {
                return Some(tt);
            }

            tmp = None;
        }

        match it.next()? {
            TokenTree::Group(g) => Some(TokenTree::Group(Group::new(
                g.delimiter(),
                process(g.stream()),
            ))),
            TokenTree::Punct(punct) => {
                if punct.as_char() == '?' {
                    let mut group = Group::new(Delimiter::Parenthesis, TokenStream::default());
                    group.set_span(punct.span());

                    tmp = Some(
                        [
                            TokenTree::Ident(Ident::new("unwrap", punct.span())),
                            TokenTree::Group(group),
                        ]
                        .into_iter(),
                    );

                    let mut first = Punct::new('.', Spacing::Joint);
                    first.set_span(punct.span());
                    Some(TokenTree::Punct(first))
                } else {
                    Some(TokenTree::Punct(punct))
                }
            }
            tt => Some(tt),
        }
    }))
}
