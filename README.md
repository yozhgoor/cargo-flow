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

## Generate

The `generate` subcommand check if a workflow file exists and its status if found.
This subcommand is no-op when not used with the `--execute` flag.
```
cargo flow generate
```

To create a new workflow file or update the existing one, use the command with the `--execute` flag.
```
cargo flow generate --execute
Workflow file created at project/.github/workflows/rust.yml
```

### Additional flags

The `generate` subcommand provide several flags to allow customization of the resulting workflow
file.

#### `name`

You can change the name of the workflow with the `--name` flag. Default to `rust`.
```
cargo flow generate --name main --execute
Workflow file created at project/.github/workflows/main.yml
```

#### `push`

Require the workflow to run on push, taking the destination branch as argument.
```
cargo flow generate --push production
```

This is equivalent to the following:
```yaml
on:
  push:
    branches:
      - production
```

The default behavior is on push to the `main` branch. To disable the default behavior, the
`--no-push` flag is available.

#### `pull-request`

Require the workflow to run on pull-requests, taking the destination branch as argument.
```
cargo flow generate --pull-request main
```

This is equivalent to the following:
```yaml
on:
  pull_request:
    branches:
      - main
```

This does not appear by default.

#### `runs-on`

Define which platform the workflow runs on. Default to `ubuntu-latest`.

```
cargo flow generate --runs-on macos-latest
```

This is equivalent to the following:
```yaml
jobs:
  checks:
    runs-on: macos-latest
```

#### `no-cache`

By default, `cargo-flow` use the [`Swatinem/rust-cache`][rust-cache] action to cache the project in
the workflow, this can be disabled using the `--no-cache` flag.

[rust-cache]: https://github.com/Swatinem/rust-cache
