use std::fs;

use anyhow::{Context, Result};
use clap::Args;
use walkdir::WalkDir;

use crate::util::repository_root;

const REPLACEMENTS: &[(&str, &str)] = &[
    ("@swc/helpers", "@swc-bun/helpers"),
    ("@swc/polyfill", "@swc-bun/polyfill"),
];

/// Known files that contain @swc/helpers or @swc/polyfill string literals.
const KNOWN_TARGETS: &[&str] = &[
    "crates/swc_ecma_transforms_base/src/helpers/mod.rs",
    "crates/swc_ecma_preset_env/src/corejs2/entry.rs",
    "crates/swc_ecma_transformer/src/es2015/typeof_symbol.rs",
    "crates/swc_ecma_minifier/src/compress/pure/misc.rs",
    "bindings/binding_core_wasm/src/types.rs",
    "bindings/binding_minifier_wasm/src/types.rs",
];

#[derive(Debug, Args)]
pub struct FixRustRefsCmd;

impl FixRustRefsCmd {
    pub fn run(self) -> Result<()> {
        let root = repository_root()?;
        let mut updated = Vec::new();

        // 1. Fix known target files
        eprintln!("  Fixing @swc/ references in known Rust source files...");
        for rel_path in KNOWN_TARGETS {
            let path = root.join(rel_path);
            if !path.exists() {
                continue;
            }

            let content =
                fs::read_to_string(&path).with_context(|| format!("failed to read {rel_path}"))?;
            let mut new_content = content.clone();

            for (from, to) in REPLACEMENTS {
                new_content = new_content.replace(from, to);
            }

            if new_content != content {
                fs::write(&path, new_content)
                    .with_context(|| format!("failed to write {rel_path}"))?;
                updated.push(rel_path.to_string());
                eprintln!("    Updated: {rel_path}");
            }
        }

        // 2. Scan for any other .rs files with @swc/helpers (upstream may add new ones)
        eprintln!("  Scanning for other Rust files with @swc/ references...");
        let mut unknown = Vec::new();

        for entry in WalkDir::new(root.join("crates"))
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != "tests" && name != "node_modules" && name != "target"
            })
            .filter_map(|e| e.ok())
        {
            if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }

            let rel = entry
                .path()
                .strip_prefix(&root)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();

            // Skip known targets (already handled)
            if KNOWN_TARGETS.contains(&rel.as_str()) {
                continue;
            }

            let content = fs::read_to_string(entry.path())?;
            if content.contains("@swc/helpers") || content.contains("@swc/polyfill") {
                unknown.push(rel);
            }
        }

        // Also scan bindings/
        for entry in WalkDir::new(root.join("bindings"))
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != "node_modules" && name != "target"
            })
            .filter_map(|e| e.ok())
        {
            if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }

            let rel = entry
                .path()
                .strip_prefix(&root)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();

            if KNOWN_TARGETS.contains(&rel.as_str()) {
                continue;
            }

            let content = fs::read_to_string(entry.path())?;
            if content.contains("@swc/helpers") || content.contains("@swc/polyfill") {
                unknown.push(rel);
            }
        }

        if !unknown.is_empty() {
            eprintln!("  Warning: found @swc/ references in files not in KNOWN_TARGETS:");
            for f in &unknown {
                eprintln!("    {f}");
            }
        }

        eprintln!("  Updated {} file(s)", updated.len());

        if !updated.is_empty() {
            eprintln!();
            eprintln!("  Test fixtures need updating. Run:");
            eprintln!("    UPDATE=1 cargo test -p swc_ecma_transforms_base");
            eprintln!("    UPDATE=1 cargo test -p swc_ecma_preset_env");
            eprintln!("    UPDATE=1 cargo test -p swc_ecma_transformer");
            eprintln!("    UPDATE=1 cargo test -p swc_ecma_minifier");
        }

        Ok(())
    }
}
