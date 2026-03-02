# AI Agent Rules

When working in a specific directory, apply the rules from that directory and all parent directories up to the root.

## While working on `.`

*Source: `AGENTS.md`*

# Instructions

## General rule

-   Read the codebase - the codebase is source of truth, and you should prefer reading lots of source code over searching.
-   Do not search web unless explicitly asked to do so. Web search does not help in general for our project.
-   Write performant code. Always prefer performance over other things.
-   Use `gh` CLI tool for fetching data from `github.com`.

## Code style

-   Write comments and documentations in English.
-   Write documentation for your code.
-   Commit your work as frequent as possible using git. Do NOT use `--no-verify` flag.
-   Prefer multiple small files over single large file.

---

-   When creating Atom instances, it's better to use `Cow<str>` or `&str` instead of `String`. Note that `&str` is better than `Cow<str>` here.

## Testing

-   Write unit tests for your code.
-   Prefer fixture tests over inline (`#[test]`) tests.
-   You can do `UPDATE=1 cargo test` to get test outputs updated for fixture tests.
-   When instructed to fix tests, do not remove or modify existing tests.

### Fixture Test Common Guide

- Add new coverage by extending existing fixture suites instead of adding ad-hoc inline tests.
- Find the exact fixture harness before adding files with: `rg -n "#\[(testing::)?fixture\(" tests src --glob "*.rs"`.
- Keep each suite's naming conventions (for example: input.*, output.*, exec.*, .ans).
- For snapshot-style tests, update expected outputs with `UPDATE=1 cargo test -p ...` using the exact crate command documented in each crate's AGENTS.md.
- Always rerun the same crate tests without `UPDATE` before finishing.

## Verification

-   Before finishing a task, always run this baseline locally.
    -   `cargo fmt --all`
    -   `cargo clippy --all --all-targets -- -D warnings`
-   For each touched Rust crate, run crate-level verification locally.
    -   `cargo test -p <crate>`
-   If wasm binding packages are touched, run:
    -   `(cd bindings/binding_core_wasm && ./scripts/test.sh)`
    -   `(cd bindings/binding_minifier_wasm && ./scripts/test.sh)`
    -   `(cd bindings/binding_typescript_wasm && ./scripts/test.sh)`
    -   `(cd bindings/binding_es_ast_viewer && ./scripts/test.sh)`
-   If node bindings or integration paths are touched, run:
    -   `(cd packages/core && yarn build:dev && yarn test)`

## Compatibility rule

-   Do not use unstable, nightly only features of rustc.


---

## While working on `crates/jsdoc`

*Source: `crates/jsdoc/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixtures.
- Update generated fixture outputs with: UPDATE=1 cargo test -p jsdoc.
- Verify without UPDATE before finishing: cargo test -p jsdoc.


---

## While working on `crates/swc`

*Source: `crates/swc/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/errors, tests/exec, tests/minify, tests/typescript, tests/vercel, tests/stacktrace, tests/babel-exec.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc.
- Verify without UPDATE before finishing: cargo test -p swc.


---

## While working on `crates/swc_bundler`

*Source: `crates/swc_bundler/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/deno-exec.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_bundler.
- Verify without UPDATE before finishing: cargo test -p swc_bundler.


---

## While working on `crates/swc_core`

*Source: `crates/swc_core/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_core.
- Verify without UPDATE before finishing: cargo test -p swc_core.


---

## While working on `crates/swc_css_codegen`

*Source: `crates/swc_css_codegen/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/options, ../swc_css_parser/tests/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_css_codegen.
- Verify without UPDATE before finishing: cargo test -p swc_css_codegen.


---

## While working on `crates/swc_css_compat`

*Source: `crates/swc_css_compat/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/nesting, tests/custom-media-query, tests/media-query-ranges, tests/color-hex-alpha, tests/color-legacy, tests/selector-not, tests/color-hwb, tests/all.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_css_compat.
- Verify without UPDATE before finishing: cargo test -p swc_css_compat.


---

## While working on `crates/swc_css_lints`

*Source: `crates/swc_css_lints/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/rules/pass, tests/rules/fail.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_css_lints.
- Verify without UPDATE before finishing: cargo test -p swc_css_lints.


---

## While working on `crates/swc_css_minifier`

*Source: `crates/swc_css_minifier/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_css_minifier.
- Verify without UPDATE before finishing: cargo test -p swc_css_minifier.


---

## While working on `crates/swc_css_modules`

*Source: `crates/swc_css_modules/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/with-compat.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_css_modules.
- Verify without UPDATE before finishing: cargo test -p swc_css_modules.


---

## While working on `crates/swc_css_parser`

*Source: `crates/swc_css_parser/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/line-comment, tests/recovery, tests/recovery-cssmodules.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_css_parser.
- Verify without UPDATE before finishing: cargo test -p swc_css_parser.


---

## While working on `crates/swc_css_prefixer`

*Source: `crates/swc_css_prefixer/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_css_prefixer.
- Verify without UPDATE before finishing: cargo test -p swc_css_prefixer.


---

## While working on `crates/swc_ecma_codegen`

*Source: `crates/swc_ecma_codegen/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/str-lits, ../swc_ecma_parser/tests/test262-parser/pass.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_codegen.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_codegen.


---

## While working on `crates/swc_ecma_lints`

*Source: `crates/swc_ecma_lints/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/pass.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_lints.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_lints.


---

## While working on `crates/swc_ecma_minifier`

*Source: `crates/swc_ecma_minifier/AGENTS.md`*

### Instructions

- You can run execution tests by doing ./scripts/exec.sh to see if your changes are working.
- If an execution test fails, you are wrong.
- Always run execution tests after making changes.
- You can run fixture tests by doing ./scripts/test.sh, and you can do UPDATE=1 ./scripts/test.sh to update fixtures.

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/terser, tests/mangle, tests/pass-1, tests/pass-default, tests/full, tests/projects, benches/full.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_minifier.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_minifier.


---

## While working on `crates/swc_ecma_parser`

*Source: `crates/swc_ecma_parser/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/js, tests/jsx, tests/typescript, tests/typescript-errors, tests/errors, tests/comments, tests/span, tests/shifted.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_parser.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_parser.


---

## While working on `crates/swc_ecma_preset_env`

*Source: `crates/swc_ecma_preset_env/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixtures.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_preset_env.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_preset_env.

## While working on `crates/swc_ecma_transformer`

*Source: `crates/swc_ecma_transformer/AGENTS.md`*

### Instructions

-   The Transformer must implement `VisitMut` and execute the `VisitMutHooks` of its subtypes.
-   Other types like ES20xx or transforms for specific syntax MUST NOT implement `VisitMut`.
-   Subtypes must implement `VisitMutHook<TraverseCtx>`.
-   Before starting work, read `$repositoryRoot/crates/swc_ecma_hooks/src/`.


---

## While working on `crates/swc_ecma_transforms`

*Source: `crates/swc_ecma_transforms/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms.


---

## While working on `crates/swc_ecma_transforms_base`

*Source: `crates/swc_ecma_transforms_base/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/resolver, tests/ts-resolver, ../swc_ecma_parser/tests.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms_base.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms_base.


---

## While working on `crates/swc_ecma_transforms_compat`

*Source: `crates/swc_ecma_transforms_compat/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/arrow, tests/async-to-generator, tests/block-scoping, tests/class-properties, tests/class_fields_use_set, tests/classes, tests/destructuring, tests/for-of, tests/new-target, tests/object-rest-spread, tests/optional-chaining, tests/optional-chaining-loose, tests/parameters, tests/private-in-object, tests/shorthand_properties, tests/static-blocks.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms_compat.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms_compat.


---

## While working on `crates/swc_ecma_transforms_module`

*Source: `crates/swc_ecma_transforms_module/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/paths.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms_module.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms_module.


---

## While working on `crates/swc_ecma_transforms_optimization`

*Source: `crates/swc_ecma_transforms_optimization/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/const-modules, tests/dce, tests/dce-jsx, tests/expr-simplifier.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms_optimization.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms_optimization.


---

## While working on `crates/swc_ecma_transforms_proposal`

*Source: `crates/swc_ecma_transforms_proposal/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/decorators, tests/explicit-resource-management.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms_proposal.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms_proposal.


---

## While working on `crates/swc_ecma_transforms_react`

*Source: `crates/swc_ecma_transforms_react/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/jsx/fixture, tests/integration/fixture, tests/script/jsx/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms_react.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms_react.


---

## While working on `crates/swc_ecma_transforms_typescript`

*Source: `crates/swc_ecma_transforms_typescript/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, ../swc_ecma_parser/tests/tsc, ../swc_ecma_parser/tests/typescript.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ecma_transforms_typescript.
- Verify without UPDATE before finishing: cargo test -p swc_ecma_transforms_typescript.


---

## While working on `crates/swc_error_reporters`

*Source: `crates/swc_error_reporters/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_error_reporters.
- Verify without UPDATE before finishing: cargo test -p swc_error_reporters.


---

## While working on `crates/swc_estree_compat`

*Source: `crates/swc_estree_compat/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixtures, tests/flavor/acorn/fixtures.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_estree_compat.
- Verify without UPDATE before finishing: cargo test -p swc_estree_compat.


---

## While working on `crates/swc_html_codegen`

*Source: `crates/swc_html_codegen/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/document_fragment, tests/options, ../swc_html_parser/tests/fixture.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_html_codegen.
- Verify without UPDATE before finishing: cargo test -p swc_html_codegen.


---

## While working on `crates/swc_html_minifier`

*Source: `crates/swc_html_minifier/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/document_fragment, tests/recovery.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_html_minifier.
- Verify without UPDATE before finishing: cargo test -p swc_html_minifier.


---

## While working on `crates/swc_html_parser`

*Source: `crates/swc_html_parser/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/recovery, tests/iframe_srcdoc, tests/html5lib-tests-fixture, tests/html5lib-tests.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_html_parser.
- Verify without UPDATE before finishing: cargo test -p swc_html_parser.


---

## While working on `crates/swc_node_bundler`

*Source: `crates/swc_node_bundler/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/pass.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_node_bundler.
- Verify without UPDATE before finishing: cargo test -p swc_node_bundler.


---

## While working on `crates/swc_plugin_backend_tests`

*Source: `crates/swc_plugin_backend_tests/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, ../swc_ecma_parser/tests/tsc.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_plugin_backend_tests.
- Verify without UPDATE before finishing: cargo test -p swc_plugin_backend_tests.


---

## While working on `crates/swc_ts_fast_strip`

*Source: `crates/swc_ts_fast_strip/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/errors, ../swc_ecma_parser/tests/tsc.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_ts_fast_strip.
- Verify without UPDATE before finishing: cargo test -p swc_ts_fast_strip.


---

## While working on `crates/swc_typescript`

*Source: `crates/swc_typescript/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/**/*.ts, tests/**/*.tsx.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_typescript.
- Verify without UPDATE before finishing: cargo test -p swc_typescript.


---

## While working on `crates/swc_xml_codegen`

*Source: `crates/swc_xml_codegen/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/options, ../swc_xml_parser/tests/fixture, ../swc_xml_parser/tests/recovery.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_xml_codegen.
- Verify without UPDATE before finishing: cargo test -p swc_xml_codegen.


---

## While working on `crates/swc_xml_parser`

*Source: `crates/swc_xml_parser/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/fixture, tests/recovery.
- Update generated fixture outputs with: UPDATE=1 cargo test -p swc_xml_parser.
- Verify without UPDATE before finishing: cargo test -p swc_xml_parser.


---

## While working on `crates/testing_macros`

*Source: `crates/testing_macros/AGENTS.md`*

### Fixture Test Addition Guide

- Preferred fixture roots in this crate: tests/simple, tests/ignore.
- Update generated fixture outputs with: UPDATE=1 cargo test -p testing_macros.
- Verify without UPDATE before finishing: cargo test -p testing_macros.

---

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
