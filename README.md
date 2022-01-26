# Errant

A (mostly) drop-in replacement for Rust's `Result` that provides backtrace support.

*Please note that Errant is still very early in development and is not yet ready for general use.*

## Example

Here's an example of errant being used to trace a runtime error.

```rs
use errant::prelude::*;

fn baz(_: i32) -> Result<i32, &'static str> {
    Err("Uh oh!")
}

fn bar(x: i32, y: i32) -> Result<i32, &'static str> {
    Ok(baz(x)? + y)
}

fn foo(x: i32) -> Result<i32, &'static str> {
    Ok(bar(x, 3)? * 2)
}

fn main() -> Result<(), &'static str> {
    foo(3)?;
    Ok(())
}
```

The result of this program is as follows (color omitted).

```
Error: "Uh oh!"
    ╭─[examples/hello.rs:1:1]
    │
  4 │     Err("Uh oh!")
    ·     ┬
    ·     ╰── Error encountered here
  8 │     Ok(baz(x)? + y)
    ·        ┬
    ·        ╰── (1) Then propagated here
 12 │     Ok(bar(x, 3)? * 2)
    ·        ┬
    ·        ╰── (2) Then propagated here
 16 │     foo(3)?;
    ·     ┬
    ·     ╰── (3) Then propagated here
────╯
```

Give it a go with the following command.

```
cargo run --example hello
```

## Design

Errant provides a replacement for `std`'s `Result<T, E>` type that automatically tracks error propagation through a
program, generating a backtrace. In addition, it also provides a variety of error wrapper types that can be used to
embellish errors with extra context when desired, such as the `Backtrace<E>` error. When a panic occurs, Errant will
display a backtrace of the error that includes useful information like propagation locations, context provided along
the way, and more.

## Philosophy

- Errors should be types, not trait objects
- Errors should point to their source and show their path through the program
- Errors should be zero-cost, requiring no (or very little) overhead in release mode

## License

`errant` is distributed under either of:

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
