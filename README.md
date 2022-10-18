# hodoku

[<img alt="github" src="https://img.shields.io/badge/github-udoprog/hodoku-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/hodoku)
[<img alt="crates.io" src="https://img.shields.io/crates/v/hodoku.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/hodoku)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-hodoku-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/hodoku)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/udoprog/hodoku/CI/main?style=for-the-badge" height="20">](https://github.com/udoprog/hodoku/actions?query=branch%3Amain)

A simple set of macros to aid testing with try operations.

This crate allows for easily writing functions and expression where `?` is
automatically translated into `.unwrap()`.

It is syntactically desirable to use `?`. This however causes issues during
testing, because a failing test lacks a stack trace which helps you track
down the exact line that errored.

```rust
#[test]
fn test_case() -> Result<(), &'static str> {
    let value = function()?;
    assert_eq!(value, 42);
    Ok(())
}
```

By default you'd get this when `function()?` errors:

```text
---- test_case stdout ----
Error: "bad"
thread 'test_case' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: the test returned a termination value with a non-zero status code (1) which indicates a failure', <path>\library\test\src\lib.rs:185:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_case
```

Note how there's no information on which line the test failed on.

But with the inclusion of `#[hodoku::function]` you get this:

```rust
#[test]
#[hodoku::function]
fn test_case() -> Result<(), &'static str> {
    let value = function()?;
    assert_eq!(value, 42);
    Ok(())
}
```

```text
---- test_case stdout ----
thread 'test_case' panicked at 'called `Result::unwrap()` on an `Err` value: "bad"', tests\failing.rs:8:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_case
```

This is exactly why we want to make use of `.unwrap()` instead of the try
operator tests. It indicates the exact line that errored.

<br>

## Examples

Use of `#[hodoku::function]`.

```rust
#[hodoku::function]
fn hello() {
    let value = Some(42)?;
    assert_eq!(value, 42);
}

hello();
```

Unwrapping expressions:

```rust
let value = hodoku::expr!(Some(42)?);
assert_eq!(value, 42);
```
