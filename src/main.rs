ocaml::import! {
    fn incr2(i: ocaml::Int) -> ocaml::Int;
}

fn main() {
    let gc = ocaml::runtime::init();
    ocaml::initial_setup();

    unsafe {
        println!("incr2 {}", incr2(&gc, 10).unwrap());
    }
}
