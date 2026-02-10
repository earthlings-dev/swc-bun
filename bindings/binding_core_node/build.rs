use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

extern crate napi_build;

fn main() {
    #[cfg(all(
        not(feature = "allow_feature_conflicts"),
        not(feature = "swc_v1"),
        not(feature = "swc_v2")
    ))]
    println!("cargo:warning=Please enable swc_v1 or swc_v2 feature");

    #[cfg(all(
        not(feature = "allow_feature_conflicts"),
        feature = "swc_v1",
        feature = "swc_v2"
    ))]
    println!("cargo:warning=Features swc_v1 and swc_v2 are incompatible");

    let out_dir = env::var("OUT_DIR").expect("Outdir should exist");
    let dest_path = Path::new(&out_dir).join("triple.txt");
    let mut f =
        BufWriter::new(File::create(dest_path).expect("Failed to create target triple text"));
    write!(
        f,
        "{}",
        env::var("TARGET").expect("Target should be specified")
    )
    .expect("Failed to write target triple text");

    napi_build::setup();
}
