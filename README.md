# Rust

## History
In May 2015, Rust 1.0 was established "stability without stagnation" as a core Rust axiom.

What does stability mean?
- Rust will keep evolving 
- Upgrading Rust to 1.X won't break your code in 1.0

What does stagnation mean?
- New work goes directly into the master branch
- Each day the last successful build from master becomes the nightly release
- Every 6 weeks, a beta branch is created off master and the previous beta
 is promoted to the new stable release

Adding a backwards incompatible change
- Sometimes it may be useful to introduce a backwards incompatible change like
adding the `async` and `await` keywords. This would result in issues with code
in earlier versions of Rust that use async and await. `let async = 1;`.
- Rust solves this with `editions`
- Each crate chooses its edition in the `Cargo.toml` file
