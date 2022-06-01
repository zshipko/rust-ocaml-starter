ocaml::import! {
    fn hello_world() -> String;
}

fn main() {
    let gc = ocaml::runtime::init();

    unsafe {
        println!("{}", hello_world(&gc).unwrap());
    }
}
