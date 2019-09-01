# Contributing to tranche

## Running tests

```
cargo test --all
cargo test --all --features std
cargo miri test --target-dir target/miri --features std
```

## Adding tests

Tests should be either doctests in `tranche` or integration tests in
`cutting_board`. Unit tests are forbidden everywhere. Tests needing additional
dependencies should only live in `cutting_board`.

## Help needed

There are very few tests and that's bad. Documentation is lacking. Continuous
integration needs to be set up.
