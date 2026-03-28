use std::{fs, process::Command};

use anyhow::{Context, Result, bail};
use clap::Args;

use crate::util::repository_root;

#[derive(Debug, Args)]
pub struct RegenerateLockfilesCmd;

impl RegenerateLockfilesCmd {
    pub fn run(self) -> Result<()> {
        let root = repository_root()?;

        // Cargo.lock
        eprintln!("  Regenerating Cargo.lock...");
        let cargo_lock = root.join("Cargo.lock");
        if cargo_lock.exists() {
            fs::remove_file(&cargo_lock).context("failed to remove Cargo.lock")?;
        }

        let status = Command::new("cargo")
            .args(["check", "--workspace"])
            .current_dir(&root)
            .status()
            .context("failed to spawn cargo check")?;

        if !status.success() {
            // Fall back to cargo update
            eprintln!("  cargo check failed, trying cargo update...");
            let status = Command::new("cargo")
                .arg("update")
                .current_dir(&root)
                .status()
                .context("failed to spawn cargo update")?;

            if !status.success() {
                bail!("failed to regenerate Cargo.lock");
            }
        }
        eprintln!("  Cargo.lock regenerated");

        // bun.lock
        eprintln!("  Regenerating bun.lock...");
        for lockfile in ["bun.lock", "yarn.lock"] {
            let path = root.join(lockfile);
            if path.exists() {
                fs::remove_file(&path).with_context(|| format!("failed to remove {lockfile}"))?;
            }
        }

        let status = Command::new("bun")
            .arg("install")
            .current_dir(&root)
            .status()
            .context("failed to spawn bun install")?;

        if !status.success() {
            bail!("bun install failed");
        }
        eprintln!("  bun.lock regenerated");

        Ok(())
    }
}
