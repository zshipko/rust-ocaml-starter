// Read the corresponding .ml file first.

////////////////////////////////////////////////////////////////////////////////
// The simple hello-world function.
ocaml::import! {
    fn hello_world() -> String;
}

////////////////////////////////////////////////////////////////////////////////
// The variant example.
#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub enum T {
    A,
    B(ocaml::Int),
}

ocaml::import! {
    fn maybe_inc(t: T) -> T;
}

////////////////////////////////////////////////////////////////////////////////
// The first struct example.
// NOTE: If the OCaml type `MyStruct.t` included a value we couldn't (or didn't
// want to) wrap, we would use `ocaml::Value` for it.
#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub struct MyStructT {
    a: ocaml::Int,
    b: ocaml::Float,
}

ocaml::import! {
    fn mystruct_inc_both(t: MyStructT) -> MyStructT;
}

////////////////////////////////////////////////////////////////////////////////
// The tuple example.

ocaml::import! {
    fn tuple_inc_both(t: (ocaml::Int, ocaml::Float)) -> (ocaml::Int, ocaml::Float);
}

////////////////////////////////////////////////////////////////////////////////
// The opaque type example.
#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub struct OpaqueT {
    oc: ocaml::Value,
}

ocaml::import! {
    fn opaque_create() -> OpaqueT;
    // We can optionally make input args references, allowing us to reuse
    // a value in Rust.
    fn opaque_use(t: &OpaqueT) -> String;
}

////////////////////////////////////////////////////////////////////////////////
// The BrokenStruct example.
#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub struct BrokenStructT {
    a: ocaml::Float,
    b: ocaml::Float,
}

ocaml::import! {
    fn brokenstruct_inc_both(t: BrokenStructT) -> BrokenStructT;
}

fn main() {
    let gc = ocaml::runtime::init();

    unsafe {
        println!("Simple example: {}", hello_world(&gc).unwrap());
    }

    unsafe {
        println!("Variant example: {:?}", maybe_inc(&gc, T::B(1)).unwrap());
    }

    unsafe {
        println!("First struct example: {:?}", mystruct_inc_both(&gc, MyStructT{a: 1, b: 2.0}).unwrap());
    }

    unsafe {
        println!("Tuple example: {:?}", tuple_inc_both(&gc, (1, 2.0)).unwrap());
    }

    unsafe {
        let opaque = opaque_create(&gc).unwrap();
        println!("Opaque type example: {:?}", opaque_use(&gc, &opaque).unwrap());
        println!("Opaque type example called again: {:?}", opaque_use(&gc, &opaque).unwrap());
    }

    unsafe {
        println!("BrokenStruct example: {:?}", brokenstruct_inc_both(&gc, BrokenStructT{a: 1.0, b: 2.0}).unwrap());
    }
}
