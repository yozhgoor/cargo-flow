# cargo-flow

A CLI helper to manager Rust project's workflows efficiently.

## Usage

```
cargo flow
```

The default behavior run several commands on a rust project:
- `cargo check`
- `cargo build`
- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --tests -- -D warnings`

`cargo-flow` will automatically detect features and workspaces and add the corresponding flags to
those commands.

### Additional flags

- `clean` enables `cargo clean` at the start of the process, ensuring no compiled artifact will
interfere with the next checks.

```
cargo flow --clean
```

- `lints` enables the `clippy::pedantic`, `clippy::restriction` and `clippy::cargo` groups for even
  more lints.

```
cargo flow --lints
```
