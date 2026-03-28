use std::{
    process::Command,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use clap::Args;

use crate::util::repository_root;

struct CheckResult {
    name: String,
    passed: bool,
    _duration: Duration,
}

#[derive(Debug, Args)]
pub struct VerifyCmd {
    /// Run extended verification (includes a subset of tests)
    #[clap(long)]
    extended: bool,
}

impl VerifyCmd {
    pub fn run(self) -> Result<()> {
        let root = repository_root()?;
        let mut results: Vec<CheckResult> = Vec::new();

        let checks: Vec<(&str, Vec<&str>)> = {
            let mut v = vec![
                ("cargo check --workspace", vec!["check", "--workspace"]),
                ("cargo fmt --check", vec!["fmt", "--all", "--", "--check"]),
                (
                    "cargo clippy",
                    vec!["clippy", "--all", "--all-targets", "--", "-D", "warnings"],
                ),
            ];

            if self.extended {
                v.push((
                    "cargo test -p swc_ecma_parser",
                    vec!["test", "-p", "swc_ecma_parser", "--no-fail-fast"],
                ));
                v.push((
                    "cargo test -p swc_ecma_codegen",
                    vec!["test", "-p", "swc_ecma_codegen", "--no-fail-fast"],
                ));
            }

            v
        };

        for (name, args) in &checks {
            eprintln!("  Running {name}...");
            let start = Instant::now();

            let status = Command::new("cargo")
                .args(args)
                .current_dir(&root)
                .status()
                .with_context(|| format!("failed to spawn: {name}"))?;

            let duration = start.elapsed();
            let passed = status.success();

            if passed {
                eprintln!("  \x1b[32m✓\x1b[0m {name} ({:.1}s)", duration.as_secs_f64());
            } else {
                eprintln!("  \x1b[31m✗\x1b[0m {name} ({:.1}s)", duration.as_secs_f64());
            }

            results.push(CheckResult {
                name: name.to_string(),
                passed,
                _duration: duration,
            });
        }

        // Summary
        eprintln!();
        let pass_count = results.iter().filter(|r| r.passed).count();
        let fail_count = results.iter().filter(|r| !r.passed).count();

        if fail_count == 0 {
            eprintln!("  \x1b[32m✓ All {pass_count} check(s) passed\x1b[0m");
            Ok(())
        } else {
            eprintln!("  \x1b[31m✗ {fail_count} check(s) FAILED\x1b[0m ({pass_count} passed)");
            for r in &results {
                if !r.passed {
                    eprintln!("    FAILED: {}", r.name);
                }
            }
            bail!("{fail_count} verification check(s) failed");
        }
    }
}
