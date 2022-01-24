# Errant

A (mostly) drop-in replacement for Rust's `Result` that provides backtrace support.

## Demo

Give it a go with

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

at the disgression of the user.
