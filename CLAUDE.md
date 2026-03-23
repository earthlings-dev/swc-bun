# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

SWC (Speedy Web Compiler) — a high-performance TypeScript/JavaScript compiler written in Rust. This is the **swc-bun fork**, migrated to Rust edition 2024. The codebase is a large Cargo workspace (~120 crates in `crates/`, plus bindings, plugins, and tools).

### Fork-Specific: Patched Dependencies

This fork uses `[patch.crates-io]` in the root `Cargo.toml` to redirect ~60 crates to local sibling repos under `../` (e.g., `../serde/serde`, `../tokio/tokio`, `../anyhow`, `../dudykr-ddbase/crates/par-core`). If builds fail with missing path errors, ensure those sibling repos exist and are checked out.

## Initial Setup

```bash
git submodule update --init --recursive   # Test suites (test262, html5lib, decorators)
bun install                               # JS dependencies

# Required environment for tests
export RUST_BACKTRACE=full
export RUST_MIN_STACK=16777216            # 16MB stack — recursive visitors need this
export PATH="$PATH:$PWD/node_modules/.bin"
```

**Toolchain**: Rust 1.94 pinned via `rust-toolchain.toml` (includes rustfmt, clippy, wasm32-wasip1/wasip2 targets). Edition 2024, MSRV 1.94. Bun >= 1.3.10.

## Common Commands

### Build

```bash
cargo check --workspace              # Fast type-check
cargo build -p <crate_name>          # Build a specific crate
```

### Format & Lint (required before every commit)

```bash
cargo fmt --all
cargo clippy --all --all-targets -- -D warnings
```

### Test

```bash
cargo test -p <crate_name>                    # Test a specific crate
cargo test -p <crate_name> <test_name>        # Single test by name
cargo test -p <crate_name> --test <file>      # Single test file

# Update fixture snapshots, then verify
UPDATE=1 cargo test -p <crate_name>
cargo test -p <crate_name>                    # Must pass without UPDATE

# Full workspace tests
cargo test --all --no-default-features --features swc_v1 --features filesystem_cache
```

### NPM Package Build & JS Tests

```bash
cd packages/core && bun run build:dev         # Debug build (dev)
cd packages/core && bun run build             # Release build

# From repo root:
bun test:core                                 # packages/core
bun test:minifier                             # packages/minifier
bun test:html                                 # packages/html
bun test:react-compiler                       # packages/react-compiler
```

### Minifier (`crates/swc_ecma_minifier`)

```bash
./crates/swc_ecma_minifier/scripts/exec.sh              # Execution tests (failure = your code is wrong)
./crates/swc_ecma_minifier/scripts/test.sh               # Fixture tests
UPDATE=1 ./crates/swc_ecma_minifier/scripts/test.sh      # Update fixtures
```

### Cargo Aliases (`.cargo/config.toml`)

`cargo codegen` (code generation), `cargo xtask` (tasks), `cargo bump` (version bumping)

## Architecture

### Core Pipeline

```
Source → swc_ecma_parser → AST → transforms → swc_ecma_codegen → Output
```

1. **`swc_ecma_parser`** — Parses JS/TS into AST
2. **`swc_ecma_transforms_base`** — Three foundational transforms:
   - **resolver** — Assigns hygiene IDs to identifiers (e.g., `a#0`, `a#1` for same-named vars in different scopes)
   - **hygiene** — Renames identifiers with conflicting hygiene IDs to unique symbols
   - **fixer** — Inserts parentheses to fix operator precedence in generated AST
3. **`swc_ecma_transforms_*`** — Feature transforms (compat, module, typescript, react, etc.)
4. **`swc_ecma_minifier`** — Minification engine
5. **`swc_ecma_codegen`** — AST back to JS source code

### Key Supporting Crates

- **`swc_atoms`** — String interning (backed by `hstr`). Prefer `&str` > `Cow<str>` > `String` when creating `Atom` instances.
- **`swc_common`** — Span, hygiene, error reporting, visitor pattern traits (`Visit`, `Fold`, `VisitMut`)
- **`swc_ecma_ast`** — AST node definitions for JS/TS
- **`swc_ecma_hooks`** — Hook-based visitor infrastructure for the new transformer

### Transformer Architecture (`crates/swc_ecma_transformer`)

- The main `Transformer` implements `VisitMut` and dispatches to subtypes via `VisitMutHooks`
- Individual transforms (ES20xx, syntax-specific) must NOT implement `VisitMut` directly
- Subtypes implement `VisitMutHook<TraverseCtx>`
- Read `crates/swc_ecma_hooks/src/` before working on transforms

### Bindings

| Rust Binding Crate | NPM Package | Purpose |
|---|---|---|
| `binding_core_node` | `packages/core` (`@swc/core`) | Main compiler |
| `binding_minifier_node` | `packages/minifier` (`@swc/minifier`) | Standalone minifier |
| `binding_html_node` | `packages/html` (`@swc/html`) | HTML compiler |
| `binding_react_compiler_node` | `packages/react-compiler` (`@swc/react-compiler`) | React compiler |

WASM bindings are in `bindings/binding_*_wasm/`, tested via `./scripts/test.sh` in each.

### Macro System

- `string_enum` — Derives string-based enums
- `ast_node` — Derives AST node boilerplate
- `parser_macros` / `codegen_macros` — Domain-specific macros (intentionally break macro hygiene)

## Code Rules

- **Performance first.** Always prefer performance over other concerns.
- **No nightly-only rustc features.** All code must compile on stable Rust.
- **Prefer small files** over large monolithic ones.
- **Don't modify existing tests** when fixing bugs — add new tests instead.
- **Prefer fixture tests** over inline `#[test]` tests. Find harnesses with: `rg -n "#\[(testing::)?fixture\(" tests src --glob "*.rs"`.
- **Commit frequently.** Never use `--no-verify`.
- Use `gh` CLI for fetching data from GitHub.

### Commit Message Format

```
<type>(<scope>): <description>

fix(es/minifier): Prevent convert_tpl_to_str when there's emoji under es5
refactor(es/typescript): Run typescript transform in two passes
test(es/minifier): Add execution tests for issue #11517
```

Scopes use `es/` prefix for ECMAScript crates (e.g., `es/minifier`, `es/parser`).

### Changeset Format

```markdown
---
swc_core: patch
swc_ecma_transforms_base: patch
---

fix(es/renamer): Check `preserved` in normal renaming mode
```

## Per-Crate Instructions

Each crate with specific rules has its own `AGENTS.md` (auto-loaded by Claude Code when working in that directory). These contain fixture test roots, update commands, and crate-specific instructions. Do not duplicate them here.

## Lint Configuration

- **clippy.toml**: `cognitive-complexity-threshold = 50`, `type-complexity-threshold = 25000`
- Interior mutability types excluded from lint: `Bytes`, `Atom`, `JsWord`, `Id`
- Variable names matching primitive types (bool, char, u32, etc.) are disallowed by clippy
- **`.rustfmt.toml`**: `group_imports = "StdExternalCrate"`, `imports_granularity = "Crate"`, edition set to 2018 (for rustfmt compatibility only — workspace edition is 2024)

## Build Profiles

Performance-critical crates (`swc_ecma_parser`, `swc_ecma_minifier`, `swc_ecma_ast`, transforms) use `opt-level = 3` even in dev/test profiles. Less critical crates use `opt-level = "s"`. Release profile uses LTO, 1 codegen unit, panic=abort, and symbol stripping.
