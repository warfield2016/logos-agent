use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(&crate_dir)
        .join("..")
        .join("..")
        .join("target")
        .join("include");

    std::fs::create_dir_all(&out_dir).expect("create include dir");

    let header_path = out_dir.join("agent_core.h");

    let config = cbindgen::Config::from_file(PathBuf::from(&crate_dir).join("cbindgen.toml"))
        .expect("load cbindgen.toml");

    match cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
    {
        Ok(b) => {
            b.write_to_file(&header_path);
            println!("cargo:rerun-if-changed=cbindgen.toml");
            println!("cargo:rerun-if-changed=src/lib.rs");
            println!("cargo:rerun-if-changed=src/ffi.rs");
            println!("cargo:warning=Generated {}", header_path.display());
        }
        Err(e) => {
            // Don't fail the build if cbindgen has trouble — useful during early dev.
            println!("cargo:warning=cbindgen skipped: {e}");
        }
    }
}
