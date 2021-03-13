# cheatsheet

## exercism set up

`exercism configure -w ~/dev/exercism`

## resources

- [Docs](https://doc.rust-lang.org/std/index.html)
      - command + k -> search shortcut: rustlang 'phrase' i.e. `rustlang loop`

## exercism

<!-- submit -->
`exercism submit src/lib.rs`

<!-- pull exercise -->
`exercism download --exercise=reverse-string --track=rust`

## linters

cargo fmt

cargo clippy --all-targets

## tests

// cargo test -- --ignored
// cargo test some_test
// cargo test some_test -- --ignored

`cargo test --manifest-path reverse-string/Cargo.toml -- --ignored`

`cargo test --all-features --manifest-path nth-prime/Cargo.toml -- --ignored`
