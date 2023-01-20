// Read the corresponding .ml file first.

ocaml::import! {
    fn hello_world() -> String;
}

#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub enum T {
    A,
    B(ocaml::Int),
}

ocaml::import! {
    fn maybe_inc(t: T) -> T;
}

// We can optionally make input args references, allowing us to reuse a value
// in Rust.
ocaml::import! {
    fn maybe_inc_ref(t: &T) -> T;
}

// If the OCaml type `MyStruct.t` included a value we couldn't (or didn't want
// to) wrap, we would use `ocaml::Value` for it.
#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub struct MyStructT {
    a: ocaml::Int,
    b: ocaml::Float,
}

ocaml::import! {
    fn mystruct_inc_both(t: MyStructT) -> MyStructT;
}

fn main() {
    let gc = ocaml::runtime::init();

    unsafe {
        println!("hello_world: {}", hello_world(&gc).unwrap());
        println!("maybe_inc: {:?}", maybe_inc(&gc, T::B(1)).unwrap());
        println!("maybe_inc_ref: {:?}", maybe_inc_ref(&gc, &T::B(1)).unwrap());
        println!("mystruct_inc_both: {:?}", mystruct_inc_both(&gc, MyStructT{a: 1, b: 2.0}).unwrap());
    }
}
