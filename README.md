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

## Ref

- Test setup ( unit test and integration test) <https://doc.rust-lang.org/book/ch11-00-testing.html>
