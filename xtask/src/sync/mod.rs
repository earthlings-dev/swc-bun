use anyhow::Result;
use clap::{Args, Subcommand};

mod fix_cargo_workspace;
mod fix_edition;
mod fix_rust_refs;
mod fix_toolchain;
mod format_and_lint;
mod regenerate_lockfiles;
mod verify;

use fix_cargo_workspace::FixCargoWorkspaceCmd;
use fix_edition::FixEditionCmd;
use fix_rust_refs::FixRustRefsCmd;
use fix_toolchain::FixToolchainCmd;
use format_and_lint::FormatAndLintCmd;
use regenerate_lockfiles::RegenerateLockfilesCmd;
use verify::VerifyCmd;

#[derive(Debug, Args)]
pub struct SyncCmd {
    #[clap(subcommand)]
    cmd: SyncSubCmd,
}

#[derive(Debug, Subcommand)]
enum SyncSubCmd {
    /// Fix root Cargo.toml: resolver, edition, workspace members,
    /// [patch.crates-io]
    FixCargoWorkspace(FixCargoWorkspaceCmd),
    /// Fix edition 2024 in all member Cargo.toml files + apply edition lint
    /// fixes
    FixEdition(FixEditionCmd),
    /// Fix @swc/helpers → @swc-bun/helpers in Rust source (not test fixtures)
    FixRustRefs(FixRustRefsCmd),
    /// Write canonical rust-toolchain.toml
    FixToolchain(FixToolchainCmd),
    /// Delete and regenerate Cargo.lock and bun.lock
    RegenerateLockfiles(RegenerateLockfilesCmd),
    /// Run cargo fmt + clippy --fix + fmt again
    FormatAndLint(FormatAndLintCmd),
    /// Verify: cargo check + fmt --check + clippy
    Verify(VerifyCmd),
}

impl SyncCmd {
    pub fn run(self) -> Result<()> {
        match self.cmd {
            SyncSubCmd::FixCargoWorkspace(c) => c.run(),
            SyncSubCmd::FixEdition(c) => c.run(),
            SyncSubCmd::FixRustRefs(c) => c.run(),
            SyncSubCmd::FixToolchain(c) => c.run(),
            SyncSubCmd::RegenerateLockfiles(c) => c.run(),
            SyncSubCmd::FormatAndLint(c) => c.run(),
            SyncSubCmd::Verify(c) => c.run(),
        }
    }
}
