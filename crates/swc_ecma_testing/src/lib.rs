use std::{env, fs, path::PathBuf, process::Command};

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use testing::CARGO_TARGET_DIR;
use tracing::debug;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsExecOptions {
    /// Cache the result of the execution.
    ///
    /// If `true`, the result of the execution will be cached.
    /// Cache is not removed and it will be reused if the source code is
    /// identical.
    ///
    /// Note that this cache is stored in cargo target directory and will be
    /// removed by `cargo clean`.
    ///
    /// You can change the cache directory name by setting the
    /// `SWC_ECMA_TESTING_CACHE_DIR`
    pub cache: bool,

    /// If true, the code is treated as an ES module.
    ///
    /// Note: Bun auto-detects ESM vs CJS from syntax, so this flag is
    /// currently unused but retained for API compatibility.
    pub module: bool,

    /// The arguments passed to the JS runtime process.
    pub args: Vec<String>,
}

fn cargo_cache_root() -> PathBuf {
    env::var("SWC_ECMA_TESTING_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| CARGO_TARGET_DIR.clone())
}

/// Returns the JS runtime binary name.
///
/// Defaults to `"bun"`. Can be overridden via the `SWC_JS_RUNTIME` env var.
fn js_runtime() -> &'static str {
    static RUNTIME: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    RUNTIME.get_or_init(|| env::var("SWC_JS_RUNTIME").unwrap_or_else(|_| "bun".to_string()))
}

/// Executes `js_code` with the configured JS runtime and captures the output.
pub fn exec_node_js(js_code: &str, opts: JsExecOptions) -> Result<String> {
    if opts.cache {
        let hash = calc_hash(&format!("{:?}:{}", opts.args, js_code));
        let cache_dir = cargo_cache_root().join(".swc-js-exec-cache");
        let cache_path = cache_dir.join(format!("{hash}.stdout"));

        if let Ok(s) = fs::read_to_string(&cache_path) {
            return Ok(s);
        }

        let output = exec_node_js(
            js_code,
            JsExecOptions {
                cache: false,
                ..opts
            },
        )?;

        fs::create_dir_all(&cache_dir).context("failed to create cache directory")?;

        fs::write(&cache_path, output.as_bytes()).context("failed to write cache")?;

        return Ok(output);
    }

    debug!("Executing js runtime ({}):\n{}", js_runtime(), js_code);

    let mut c = Command::new(js_runtime());

    c.arg("-e").arg(js_code);

    for arg in opts.args {
        c.arg(arg);
    }

    let output = c.output().context("failed to execute output of minifier")?;

    if !output.status.success() {
        bail!(
            "failed to execute:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }

    String::from_utf8(output.stdout).context("output is not utf8")
}

fn calc_hash(s: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(s.as_bytes());
    let sum = hasher.finalize();

    hex::encode(sum)
}
