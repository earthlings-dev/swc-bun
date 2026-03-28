use std::fs;

use anyhow::{Context, Result};
use clap::Args;
use toml_edit::{DocumentMut, Formatted, InlineTable, Item, Value};
use walkdir::WalkDir;

use crate::util::repository_root;

const FORK_RESOLVER: &str = "3";
const FORK_EDITION: &str = "2024";
const FORK_RUST_VERSION: &str = "1.94";

const FORK_ONLY_MEMBERS: &[&str] = &[
    "extra-bindings/crates/css_node",
    "extra-bindings/crates/linter_node",
    "plugins/crates/swc_experimental_babel",
    "plugins/crates/swc_feature_flags",
    "plugins/crates/swc_icu_messageformat_parser",
    "plugins/packages/emotion",
    "plugins/packages/feature-flags",
    "plugins/packages/formatjs",
    "plugins/packages/jest",
    "plugins/packages/loadable-components",
    "plugins/packages/noop",
    "plugins/packages/prefresh",
    "plugins/packages/react-remove-properties",
    "plugins/packages/relay",
    "plugins/packages/remove-console",
    "plugins/packages/styled-components",
    "plugins/packages/styled-jsx",
    "plugins/packages/swc-confidential",
    "plugins/packages/swc-magic",
    "plugins/packages/swc-sdk",
    "plugins/packages/transform-imports",
];

const EXTERNAL_PATCHES: &[(&str, &str)] = &[
    // dudykr-ddbase
    ("par-core", "../dudykr-ddbase/crates/par-core"),
    ("par-iter", "../dudykr-ddbase/crates/par-iter"),
    ("bytes-str", "../dudykr-ddbase/crates/bytes-str"),
    ("is-macro", "../dudykr-ddbase/crates/is-macro"),
    ("shrink-to-fit", "../dudykr-ddbase/crates/shrink-to-fit"),
    (
        "shrink-to-fit-macro",
        "../dudykr-ddbase/crates/shrink-to-fit-macro",
    ),
    ("st-map", "../dudykr-ddbase/crates/st-map"),
    (
        "static-map-macro",
        "../dudykr-ddbase/crates/static-map-macro",
    ),
    // serde ecosystem
    ("serde", "../serde/serde"),
    ("serde_derive", "../serde/serde_derive"),
    ("serde_core", "../serde/serde_core"),
    ("serde_derive_internals", "../serde/serde_derive_internals"),
    ("serde_json", "../json"),
    ("schemars", "../schemars/schemars"),
    ("schemars_derive", "../schemars/schemars_derive"),
    // error handling
    ("anyhow", "../anyhow"),
    ("thiserror", "../thiserror"),
    ("thiserror-impl", "../thiserror/impl"),
    // async / runtime
    ("tokio", "../tokio/tokio"),
    ("tokio-macros", "../tokio/tokio-macros"),
    ("tokio-stream", "../tokio/tokio-stream"),
    ("tokio-util", "../tokio/tokio-util"),
    ("tracing", "../tracing/tracing"),
    ("tracing-core", "../tracing/tracing-core"),
    ("tracing-attributes", "../tracing/tracing-attributes"),
    ("tracing-log", "../tracing/tracing-log"),
    ("tracing-subscriber", "../tracing/tracing-subscriber"),
    ("pin-project-lite", "../pin-project-lit"),
    // data structures / utilities
    ("bumpalo", "../bumpalo"),
    ("chrono", "../chrono"),
    ("getrandom", "../getrandom"),
    ("globset", "../ripgrep/crates/globset"),
    ("num-bigint", "../num-bigint"),
    ("rand", "../rand"),
    ("rand_core", "../rand_core"),
    ("rayon", "../rayon-rs"),
    ("rayon-core", "../rayon-rs/rayon-core"),
    ("semver", "../semver"),
    ("tempfile", "../tempfile"),
    ("toml", "../toml/crates/toml"),
    ("toml_edit", "../toml/crates/toml_edit"),
    ("toml_datetime", "../toml/crates/toml_datetime"),
    ("serde_spanned", "../toml/crates/serde_spanned"),
    ("url", "../rust-url/url"),
    ("form_urlencoded", "../rust-url/form_urlencoded"),
    ("idna", "../rust-url/idna"),
    ("percent-encoding", "../rust-url/percent_encoding"),
    // testing
    ("assert_cmd", "../assert_cmd"),
    ("assert_fs", "../assert_fs"),
    ("predicates", "../predicates-rs"),
    ("predicates-core", "../predicates-rs/crates/core"),
    ("predicates-tree", "../predicates-rs/crates/tree"),
    // networking
    ("reqwest", "../reqwest"),
    // wasmer / wasmtime
    ("regalloc2", "../regalloc2"),
    ("wasmer", "../wasmer/lib/api"),
    ("wasmer-compiler", "../wasmer/lib/compiler"),
    (
        "wasmer-compiler-cranelift",
        "../wasmer/lib/compiler-cranelift",
    ),
    ("wasmer-config", "../wasmer/lib/config"),
    ("wasmer-derive", "../wasmer/lib/derive"),
    ("wasmer-journal", "../wasmer/lib/journal"),
    ("wasmer-types", "../wasmer/lib/types"),
    ("wasmer-vm", "../wasmer/lib/vm"),
    ("wasmer-wasix", "../wasmer/lib/wasix"),
    ("wasmer-wasix-types", "../wasmer/lib/wasi-types"),
    ("virtual-fs", "../wasmer/lib/virtual-fs"),
    ("virtual-net", "../wasmer/lib/virtual-net"),
    ("virtual-mio", "../wasmer/lib/virtual-io"),
    ("wai-bindgen-wasmer", "../wasmer/lib/wai-bindgen-wasmer"),
    ("cranelift-codegen", "../wasmtime/cranelift/codegen"),
    ("cranelift-entity", "../wasmtime/cranelift/entity"),
    ("cranelift-frontend", "../wasmtime/cranelift/frontend"),
    ("cranelift-bforest", "../wasmtime/cranelift/bforest"),
    ("cranelift-bitset", "../wasmtime/cranelift/bitset"),
    ("cranelift-control", "../wasmtime/cranelift/control"),
    ("cranelift-isle", "../wasmtime/cranelift/isle/isle"),
    (
        "cranelift-codegen-meta",
        "../wasmtime/cranelift/codegen/meta",
    ),
    (
        "cranelift-codegen-shared",
        "../wasmtime/cranelift/codegen/shared",
    ),
    ("cranelift-srcgen", "../wasmtime/cranelift/srcgen"),
    (
        "cranelift-assembler-x64",
        "../wasmtime/cranelift/assembler-x64",
    ),
    (
        "cranelift-assembler-x64-meta",
        "../wasmtime/cranelift/assembler-x64/meta",
    ),
    // build tooling
    ("clang-sys", "../clang-sys"),
];

#[derive(Debug, Args)]
pub struct FixCargoWorkspaceCmd;

impl FixCargoWorkspaceCmd {
    pub fn run(self) -> Result<()> {
        let root = repository_root()?;
        let cargo_toml_path = root.join("Cargo.toml");
        let content =
            fs::read_to_string(&cargo_toml_path).context("failed to read root Cargo.toml")?;
        let mut doc: DocumentMut = content.parse().context("failed to parse root Cargo.toml")?;

        // 1. Fix resolver
        if let Some(ws) = doc.get_mut("workspace") {
            ws["resolver"] = toml_edit::value(FORK_RESOLVER);
            eprintln!("  Set resolver = \"{FORK_RESOLVER}\"");
        }

        // 2. Fix workspace.package edition & rust-version
        if let Some(ws) = doc.get_mut("workspace") {
            if let Some(pkg) = ws.get_mut("package") {
                pkg["edition"] = toml_edit::value(FORK_EDITION);
                pkg["rust-version"] = toml_edit::value(FORK_RUST_VERSION);
                eprintln!(
                    "  Set edition = \"{FORK_EDITION}\", rust-version = \"{FORK_RUST_VERSION}\""
                );
            }
        }

        // 3. Ensure fork-only workspace members
        if let Some(ws) = doc.get_mut("workspace") {
            if let Some(Item::Value(Value::Array(members))) = ws.get_mut("members") {
                let existing: Vec<String> = members
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();

                let mut added = 0usize;
                for member in FORK_ONLY_MEMBERS {
                    if !existing.iter().any(|m| m == member) {
                        members.push(*member);
                        added += 1;
                    }
                }
                if added > 0 {
                    eprintln!("  Added {added} fork-only workspace members");
                }
            }
        }

        // 4. Rebuild [patch.crates-io]
        // Remove existing
        doc.remove("patch");
        eprintln!("  Removed existing [patch.crates-io]");

        // Auto-discover SWC crates under crates/
        let mut swc_patches: Vec<(String, String)> = Vec::new();
        for entry in WalkDir::new(root.join("crates"))
            .min_depth(1)
            .max_depth(1)
            .sort_by_file_name()
        {
            let entry = entry?;
            if entry.file_type().is_dir() {
                let cargo_toml = entry.path().join("Cargo.toml");
                if cargo_toml.exists() {
                    let crate_content = fs::read_to_string(&cargo_toml)?;
                    if let Ok(crate_doc) = crate_content.parse::<DocumentMut>() {
                        if let Some(name) = crate_doc
                            .get("package")
                            .and_then(|p| p.get("name"))
                            .and_then(|n| n.as_str())
                        {
                            let dir_name = entry.file_name().to_string_lossy();
                            swc_patches.push((name.to_string(), format!("crates/{dir_name}")));
                        }
                    }
                }
            }
        }

        // Build the patch table
        let mut patch_table = toml_edit::Table::new();

        // External patches first
        for (name, path) in EXTERNAL_PATCHES {
            let mut inline = InlineTable::new();
            inline.insert("path", Value::String(Formatted::new(path.to_string())));
            patch_table.insert(name, Item::Value(Value::InlineTable(inline)));
        }

        // SWC crate patches
        for (name, path) in &swc_patches {
            let mut inline = InlineTable::new();
            inline.insert("path", Value::String(Formatted::new(path.clone())));
            patch_table.insert(name, Item::Value(Value::InlineTable(inline)));
        }

        // Insert [patch.crates-io] as a dotted table
        let mut patch_outer = toml_edit::Table::new();
        patch_outer.insert("crates-io", Item::Table(patch_table));
        doc.insert("patch", Item::Table(patch_outer));

        eprintln!(
            "  Rebuilt [patch.crates-io] with {} external + {} SWC crate patches",
            EXTERNAL_PATCHES.len(),
            swc_patches.len()
        );

        fs::write(&cargo_toml_path, doc.to_string()).context("failed to write root Cargo.toml")?;
        eprintln!("  Wrote {}", cargo_toml_path.display());

        Ok(())
    }
}
