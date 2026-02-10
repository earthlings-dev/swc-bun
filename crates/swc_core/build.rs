use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use vergen::{CargoBuilder, Emitter};

fn main() {
    // Validate conflict between host / plugin features.
    // During workspace builds with --all-features we allow both to compile, but
    // keep a warning for consumers.
    #[cfg(all(
        not(feature = "allow_feature_conflicts"),
        feature = "ecma_plugin_transform",
        any(
            feature = "plugin_transform_host_native",
            feature = "plugin_transform_host_js"
        )
    ))]
    {
        println!(
            "cargo:warning=Both 'plugin_transform' and 'plugin_transform_host*' are enabled; \
             prefer enabling only one set for production builds."
        );
    }

    let cargo = CargoBuilder::all_cargo().unwrap();

    // Creates a static compile time constants for the version of swc_core.
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    let out_dir = env::var("OUT_DIR").expect("Outdir should exist");
    let dest_path = Path::new(&out_dir).join("core_pkg_version.txt");
    let mut f = BufWriter::new(
        File::create(dest_path).expect("Failed to create swc_core version constant"),
    );
    write!(f, "{pkg_version}").expect("Failed to write swc_core version constant");

    // Attempt to collect some build time env values but will skip if there are any
    // errors.

    Emitter::default()
        .add_instructions(&cargo)
        .unwrap()
        .emit()
        .unwrap();
}
