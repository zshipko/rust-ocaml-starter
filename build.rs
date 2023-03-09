use std::{env, path::PathBuf};

pub fn run() {
    let root = {
        let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        PathBuf::from(current_dir)
    };

    assert!(std::process::Command::new("dune")
        .current_dir(&root)
        .arg("build")
        .arg("--build-dir")
        .arg("_build")
        .status()
        .unwrap()
        .success());

    let mut build = cc::Build::new();
    let path = root
        .join("_build")
        .join("default")
        .join("lib")
        .join("rust_ocaml_starter.so");
    build.object(path);

    build.compile("ocaml");
}

pub fn main() {
    // See:
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#build-scripts
    println!("cargo:rerun-if-changed=lib");
    run();
}
