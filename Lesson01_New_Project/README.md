# Lesson 01 New Project
This new project was created with `cargo new <name of project>`

```
$ cargo new foo
    Creating binary (application) `foo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

```

By default Cargo sets the project to use the latest edition.

You can confirm the edition in the `Cargo.toml` file.
```
$ cat foo/Cargo.toml
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

You can also explicitly specify the edition with `--edition <year>`. Note
cargo will not allow invalid editions.
```
$ cargo new --edition 2018 foo
    Creating binary (application) `foo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cat foo/Cargo.toml
[package]
name = "foo"
version = "0.1.0"
edition = "2018"

[dependencies]
```

## Transitioning A Project To A New Edition
Rust has tools to automatically transition a project from edition to the next.

1. Run `cargo fix --edition`
2. Edit `Cargo.toml` and set the `edition` to the latest, example `edition = "2021"`
3. Run `cargo build` or `cargo test` to verify the fixes worked

