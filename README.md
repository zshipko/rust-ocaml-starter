# rust-ocaml-starter

An example project using [ocaml-rs](https://github.com/zshipko/ocaml-rs) to link an OCaml library into a Rust program.

If you're looking for an example of calling into Rust from OCaml take a look at [ocaml-rust-starter](https://github.com/zshipko/ocaml-rust-starter)

## New project checklist

- [ ] Update `README.md`
- [ ] Update the project name in `dune-project`
- [ ] Update the crate name in `Cargo.toml`
- [ ] Update `lib/dune` and `lib/dune` with the name of your project in place of `rust_ocaml_starter`/`rust-ocaml-starter`
- [ ] Rename `rust-ocaml-starter.opam` to match the name of your project
- [ ] Remove `lib/rust_ocaml_starter.ml` add your own OCaml files
- [ ] Edit `src/lib.rs`

Many of the renaming steps can be automated using `init.sh`:

```shell
$ ./init.sh my_project my-project
```

`init.sh` accepts two arguments: the `name` of the OCaml project and the `public_name`, if they are the same then one argument is acceptable.

NOTE: `init.sh` is destructive and can only be executed once (it will remove itself) - make sure you have no modifications that you want to keep before running `init.sh`.

## Building

    cargo build

to run the program:

    cargo run

