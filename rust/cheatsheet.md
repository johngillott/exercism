# cheatsheet

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

`cargo test --all-features --manifest-path reverse-string/Cargo.toml -- --ignored`
