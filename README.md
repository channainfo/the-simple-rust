# Getting Started

## Add cargo

```sh
# to be able to edit cargo.toml file using cargo add command, install the cargo-edit crate
cargo install cargo-edit

# add a new crate called num to cargo using the cargo-edit
cargo add num

# copy crate doc to local
cargo doc
```

## Test

To run test

```sh
# run all the test
cargo test

# run filtered test with name
cargo test {name}

# possible cargo test command option
cargo test -- --help

```

## Crates

### Multiple binary crates

Run a specific binary crate:

```sh
# bin/ext.rs is a binary crate
cargo run --bin ext

# run the main.rs, which is the rollapp default to the package_name
cargo run --bin rollapp
```

### One lib crate

we have both binaray and lib crates.

- One and only one lib crate
- Multiple binary crates

## Ref

- Test setup ( unit test and integration test) <https://doc.rust-lang.org/book/ch11-00-testing.html>
- Single vs Multitple lifetimes: <https://channaly.medium.com/single-versus-multiple-lifetimes-23e3f9cad251>
