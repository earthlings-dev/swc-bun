# AI Agent Rules

When working in a specific directory, apply the rules from that directory and all parent directories up to the root.

## While working on `.`

*Source: `AGENTS.md`*

### Instructions

-   Write performant code. Always prefer performance over other things.
-   Write comments and documentations in English.
-   Do not use unstable, nightly only features of rustc.
-   When creating Atom instances, it's better to use `Cow<str>` or `&str` instead of `String`. Note that `&str` is better than `Cow<str>` here.
-   Write unit tests for your code.
-   When instructed to fix tests, do not remove or modify existing tests.
-   Write documentation for your code.
-   Run `cargo fmt --all` before commiting files.
-   Commit your work as frequent as possible using git. Do NOT use `--no-verify` flag.
-   Prefer multiple small files over single large file.

## While working on `crates/swc_ecma_minifier`

*Source: `crates/swc_ecma_minifier/AGENTS.md`*

### Instructions

-   You can run execution tests by doing `./scripts/exec.sh` to see if your changes are working.
-   If an execution test fails, you are wrong.
-   Always run execution tests after making changes.
-   You can run fixture tests by doing `./scripts/test.sh`, and you can do `UPDATE=1 ./scripts/test.sh` to update fixtures.

## While working on `crates/swc_ecma_transformer`

*Source: `crates/swc_ecma_transformer/AGENTS.md`*

### Instructions

-   The Transformer must implement `VisitMut` and execute the `VisitMutHooks` of its subtypes.
-   Other types like ES20xx or transforms for specific syntax MUST NOT implement `VisitMut`.
-   Subtypes must implement `VisitMutHook<TraverseCtx>`.
-   Before starting work, read `$repositoryRoot/crates/swc_ecma_hooks/src/`.

## Project Overview

SWC (Speedy Web Compiler) is a high-performance TypeScript/JavaScript compiler written in Rust. This is the `swc-bun` fork. The codebase is a large Cargo workspace (~118 crates in `crates/`, plus bindings and tools).

## Build and Development Commands

### Building

```bash
cargo check --workspace          # Fast type-check
cargo build -p <crate_name>      # Build a specific crate
```

### Formatting and Linting

```bash
cargo fmt --all                  # Format all code (required before committing)
cargo clippy --workspace --all-targets  # Lint check
```

Nightly-only rustfmt options are configured in `.rustfmt.toml` (group_imports, imports_granularity, etc.) but the edition is set to 2018 for compatibility.

### Running Tests

```bash
# Test a specific crate
cargo test -p <crate_name>

# Test all crates (requires environment setup, see below)
cargo test --all --no-default-features --features swc_v1 --features filesystem_cache

# Run a single test by name
cargo test -p <crate_name> <test_name>

# Run a single test file
cargo test -p <crate_name> --test <test_file_name>
```

**Required environment variables for tests:**
```bash
export RUST_BACKTRACE=full
export PATH="$PATH:$PWD/node_modules/.bin"
export RUST_MIN_STACK=16777216
```

**Initial setup for full test suite:** `git submodule update --init --recursive` (pulls test262 conformance suite), then `bun install` for JS dependencies.

### Minifier-Specific Testing (`crates/swc_ecma_minifier`)

```bash
# Execution tests (run after every change; failure means your code is wrong)
./crates/swc_ecma_minifier/scripts/exec.sh

# Fixture tests
./crates/swc_ecma_minifier/scripts/test.sh

# Update fixture snapshots
UPDATE=1 ./crates/swc_ecma_minifier/scripts/test.sh
```

The exec script runs: `cargo test -q --features concurrent --features debug --test exec --test terser_exec`

### Cargo Aliases

Defined in `.cargo/config.toml`:
- `cargo codegen` — run code generation tool
- `cargo xtask` — run xtask commands
- `cargo bump` — version bumping via swc-releaser

## Architecture

### Core Pipeline

The compiler pipeline flows through these key crates:

1. **`swc_ecma_parser`** — Parses JS/TS source into AST
2. **`swc_ecma_transforms_base`** — Three foundational transforms all other transforms depend on:
   - **resolver** — Assigns hygiene IDs to identifiers (e.g., `a#0`, `a#1` for same-named vars in different scopes)
   - **hygiene** — Renames identifiers with conflicting hygiene IDs to unique symbols
   - **fixer** — Inserts parentheses to fix operator precedence in generated AST
3. **`swc_ecma_transforms_*`** — Feature transforms (compat, module, typescript, react, etc.)
4. **`swc_ecma_minifier`** — Minification engine
5. **`swc_ecma_codegen`** — AST back to JS source code

### Supporting Crates

- **`swc_atoms`** — String interning (backed by `hstr`). Prefer `&str` > `Cow<str>` > `String` when creating `Atom` instances.
- **`swc_common`** — Span, hygiene, error reporting, visitor pattern traits (`Visit`, `Fold`, `VisitMut`)
- **`swc_ecma_ast`** — AST node definitions for JS/TS
- **`swc_ecma_hooks`** — Hook-based visitor infrastructure for the new transformer

### Macro System

SWC uses proc macros extensively:
- `string_enum` — Derives string-based enums
- `ast_node` — Derives AST node boilerplate
- `parser_macros` / `codegen_macros` — Domain-specific macros (these break macro hygiene by design)

### Transformer Architecture (`crates/swc_ecma_transformer`)

- The main `Transformer` implements `VisitMut` and dispatches to subtypes via `VisitMutHooks`
- Individual transforms (ES20xx, syntax-specific) must NOT implement `VisitMut` directly
- Subtypes implement `VisitMutHook<TraverseCtx>`
- Read `crates/swc_ecma_hooks/src/` before working on transforms

### Bindings

`bindings/` contains native NAPI and WASM bindings for using SWC from JavaScript. The JS layer is Bun-exclusive; NAPI `.node` files are loaded via Bun's Node-API compatibility.

## Code Conventions

- **Performance first.** Always prefer performance over other concerns.
- **No nightly-only rustc features.** All code must compile on stable Rust.
- **English only** for comments and documentation.
- **Prefer small files** over large monolithic ones.
- **Don't modify existing tests** when fixing bugs — add new tests instead.
- **Commit frequently.** Never use `--no-verify`.

### Commit Message Format

Follow the conventional commit style used in this repo:
```
<type>(<scope>): <description>

# Examples:
fix(es/minifier): Prevent convert_tpl_to_str when there's emoji under es5
refactor(es/typescript): Run typescript transform in two passes
test(es/minifier): Add execution tests for issue #11517
```

Scopes use `es/` prefix for ECMAScript crates (e.g., `es/minifier`, `es/typescript`, `es/parser`).

### Changeset Format

For PRs that need changelog entries:
```markdown
---
swc_core: patch
swc_ecma_transforms_base: patch
---

fix(es/renamer): Check `preserved` in normal renaming mode
```

## Build Profiles

The workspace has extensive per-crate optimization overrides. Performance-critical crates (parser, minifier, AST, transforms) use `opt-level = 3` even in dev/test profiles. Less critical crates use `opt-level = "s"`. The release profile uses LTO, 1 codegen unit, panic=abort, and symbol stripping.

## Lint Configuration

- **clippy.toml**: `cognitive-complexity-threshold = 50`, `type-complexity-threshold = 25000`, `msrv = "1.73"`
- Interior mutability types excluded from lint: `Bytes`, `Atom`, `JsWord`, `Id`
- Variable names matching primitive types (bool, char, u32, etc.) are disallowed by clippy
