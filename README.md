## Closed-form expression of Fibonacci numbers
This is done as part of my recreational programming late-night sessions and does not comply with the well-known recursion approach, but instead emphasizes Binet's formula.

## Build
This repo requires Rust language.<br>To run the program use Rust's package manager cargo.
- `cargo check` - This will essentially compile the packages without performing the final step of code generation, which is faster than running `cargo build`. The compiler will save metadata files to disk so that future runs will reuse them if the source has not been modified. Some diagnostics and errors are only emitted during code generation, so they inherently won’t be reported with `cargo check`.
- `cargo run` - Run a binary or example of the local package.

## Info
_rustc version: rustc 1.98.1 (48a229cea 2026-09-01)_
