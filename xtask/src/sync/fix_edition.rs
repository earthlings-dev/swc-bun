use std::{fs, process::Command};

use anyhow::{Context, Result};
use clap::Args;
use regex::Regex;
use toml_edit::DocumentMut;
use walkdir::WalkDir;

use crate::util::repository_root;

const FORK_EDITION: &str = "2024";
const FORK_RUST_VERSION: &str = "1.94";

#[derive(Debug, Args)]
pub struct FixEditionCmd;

impl FixEditionCmd {
    pub fn run(self) -> Result<()> {
        let root = repository_root()?;
        let mut toml_count = 0usize;
        let mut rs_count = 0usize;

        // 1. Fix edition in all Cargo.toml files
        eprintln!("  Fixing edition in Cargo.toml files...");
        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != "node_modules" && name != "target" && name != ".git"
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_name() != "Cargo.toml" {
                continue;
            }
            // Skip test fixtures
            let rel = entry.path().strip_prefix(&root).unwrap_or(entry.path());
            let rel_str = rel.to_string_lossy();
            if rel_str.contains("/tests/") && rel_str.contains("/fixture") {
                continue;
            }

            let content = fs::read_to_string(entry.path())?;
            let mut doc: DocumentMut = match content.parse() {
                Ok(d) => d,
                Err(_) => continue,
            };

            let mut changed = false;
            if let Some(pkg) = doc.get_mut("package") {
                if let Some(edition) = pkg.get("edition") {
                    if edition.as_str() == Some("2021") {
                        pkg["edition"] = toml_edit::value(FORK_EDITION);
                        changed = true;
                    }
                }
                if let Some(rv) = pkg.get("rust-version") {
                    if let Some(v) = rv.as_str() {
                        if v != FORK_RUST_VERSION && v.starts_with("1.") {
                            let minor: u32 =
                                v.strip_prefix("1.").unwrap_or("0").parse().unwrap_or(0);
                            if minor < 94 {
                                pkg["rust-version"] = toml_edit::value(FORK_RUST_VERSION);
                                changed = true;
                            }
                        }
                    }
                }
            }

            if changed {
                fs::write(entry.path(), doc.to_string())?;
                toml_count += 1;
            }
        }
        eprintln!("  Updated edition in {toml_count} Cargo.toml file(s)");

        // 2. Fix unsafe extern blocks and #[no_mangle] in .rs files
        eprintln!("  Fixing edition 2024 patterns in .rs files...");
        let extern_re = Regex::new(r#"(?m)^(\s*)extern\s+"C"\s*\{"#)?;
        let no_mangle_re = Regex::new(r"#\[no_mangle\]")?;

        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != "node_modules" && name != "target" && name != ".git"
            })
            .filter_map(|e| e.ok())
        {
            if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }

            let content = fs::read_to_string(entry.path())?;
            let mut new_content = content.clone();

            // extern "C" { → unsafe extern "C" { (only if not already unsafe)
            if extern_re.is_match(&new_content) && !new_content.contains("unsafe extern \"C\"") {
                new_content = extern_re
                    .replace_all(&new_content, "${1}unsafe extern \"C\" {")
                    .into_owned();
            }

            // #[no_mangle] → #[unsafe(no_mangle)]
            if no_mangle_re.is_match(&new_content) {
                new_content = no_mangle_re
                    .replace_all(&new_content, "#[unsafe(no_mangle)]")
                    .into_owned();
            }

            if new_content != content {
                fs::write(entry.path(), new_content)?;
                rs_count += 1;
            }
        }
        eprintln!("  Fixed edition 2024 patterns in {rs_count} .rs file(s)");

        // 3. Run cargo clippy --fix
        eprintln!("  Running cargo clippy --fix...");
        let status = Command::new("cargo")
            .args([
                "clippy",
                "--fix",
                "--allow-dirty",
                "--workspace",
                "--all-targets",
            ])
            .current_dir(&root)
            .status()
            .context("failed to spawn cargo clippy --fix")?;

        if !status.success() {
            eprintln!("  Warning: cargo clippy --fix exited with {status}");
        }

        Ok(())
    }
}
