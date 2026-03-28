use std::fs;

use anyhow::{Context, Result};
use clap::Args;

use crate::util::repository_root;

const TOOLCHAIN_CONTENT: &str = r#"[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["wasm32-wasip1", "wasm32-wasip2"]
"#;

#[derive(Debug, Args)]
pub struct FixToolchainCmd;

impl FixToolchainCmd {
    pub fn run(self) -> Result<()> {
        let path = repository_root()?.join("rust-toolchain.toml");
        fs::write(&path, TOOLCHAIN_CONTENT).context("failed to write rust-toolchain.toml")?;
        eprintln!("  Wrote rust-toolchain.toml (channel = \"stable\")");
        Ok(())
    }
}
