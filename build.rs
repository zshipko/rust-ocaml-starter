pub fn main() {
    // See:
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#build-scripts
    println!("cargo:rerun-if-changed=lib");
    ocaml_build::Dune::new("lib").build()
}
