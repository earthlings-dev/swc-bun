# AI Agent Instructions (Repository Root)

Use these rules when working anywhere in this repository unless a more specific `AGENTS.md` overrides them.

## Core Rules

- Write performant code. Always prefer performance over other things.
- Write comments and documentation in English.
- Do not use unstable, nightly-only features of `rustc`.
- When creating Atom instances, it's better to use `Cow<str>` or `&str` instead of `String`. Note that `&str` is better than `Cow<str>` here.
- Write unit tests for your code.
- When instructed to fix tests, do not remove or modify existing tests.
- Write documentation for your code.
- Run `cargo fmt --all` before committing files.
- Commit your work as frequently as possible using git. Do NOT use `--no-verify`.
- Prefer multiple small files over a single large file.

## Project Structure & Module Organization
- `crates/`: main Rust workspace crates (compiler, transforms, utilities). Most code changes land here.
- `bindings/`: native NAPI and WASM bindings for JS usage.
- `packages/`: JS packages and tooling integration.
- `tools/` and `xtask/`: developer tooling and automation.
- `scripts/`: helper scripts for tests and maintenance.
- `docs/` and `ARCHITECTURE.md`: design notes and project documentation.
- Tests typically live in `crates/<crate>/tests/` and fixture data under `crates/<crate>/tests/fixtures/` (e.g., codegen references in `crates/swc_ecma_codegen/tests/references/`).

## Build, Test, and Development Commands
- `cargo check --workspace`: fast type-check for the whole workspace.
- `cargo build -p <crate_name>`: build a specific crate.
- `cargo fmt --all`: format Rust code (required before committing).
- `cargo clippy --workspace --all-targets`: lint the workspace.
- `cargo test -p <crate_name>`: run tests for a crate.
- `cargo test -p <crate_name> --test <test_file_name>`: run a single test file.
- Full test setup: `git submodule update --init --recursive` (test262), then `bun install` for JS deps.
- Minifier exec tests: `./crates/swc_ecma_minifier/scripts/exec.sh`.
- Minifier fixture tests: `./crates/swc_ecma_minifier/scripts/test.sh`.
- Update minifier fixtures: `UPDATE=1 ./crates/swc_ecma_minifier/scripts/test.sh`.

## Coding Style & Naming Conventions
- Rust formatting is enforced by `rustfmt` (see `.rustfmt.toml`).
- Linting uses `clippy` with repo-specific thresholds in `clippy.toml`.
- Use stable Rust only; no nightly-only features.
- Prefer small files, performance-first changes, and English-only comments.
- Don’t modify existing tests when fixing bugs; add new tests instead.

## Testing Guidelines
- Required env vars for tests: `RUST_BACKTRACE=full`, `PATH="$PATH:$PWD/node_modules/.bin"`, `RUST_MIN_STACK=16777216`.
- Use `cargo test` per crate for faster iteration. Full suite requires the test262 submodule.

## Commit & Pull Request Guidelines
- Conventional commits are expected:
- `fix(es/minifier): Prevent convert_tpl_to_str when there's emoji under es5`
- `test(es/minifier): Add execution tests for issue #11517`
- Scopes use the `es/` prefix for ECMAScript crates (e.g., `es/parser`).
- Run formatting and tests relevant to your change. Avoid `--no-verify`.
- Include a clear PR description, the affected crates, and the tests you ran. Add changesets when a changelog entry is required.
