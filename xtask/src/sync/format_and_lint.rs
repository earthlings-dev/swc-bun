use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::Args;

use crate::util::repository_root;

#[derive(Debug, Args)]
pub struct FormatAndLintCmd;

impl FormatAndLintCmd {
    pub fn run(self) -> Result<()> {
        let root = repository_root()?;

        // 1. cargo fmt
        eprintln!("  Running cargo fmt --all...");
        let status = Command::new("cargo")
            .args(["fmt", "--all"])
            .current_dir(&root)
            .status()
            .context("failed to spawn cargo fmt")?;

        if !status.success() {
            bail!("cargo fmt failed");
        }

        // 2. cargo clippy --fix
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
            eprintln!(
                "  Warning: cargo clippy --fix exited with non-zero (some fixes may need manual \
                 intervention)"
            );
        }

        // 3. cargo fmt again (clippy --fix can break formatting)
        eprintln!("  Re-running cargo fmt --all...");
        let status = Command::new("cargo")
            .args(["fmt", "--all"])
            .current_dir(&root)
            .status()
            .context("failed to spawn cargo fmt")?;

        if !status.success() {
            bail!("cargo fmt (second pass) failed");
        }

        eprintln!("  Format and lint complete");
        Ok(())
    }
}
