let hello_world () = "Hello, world!"

let () = Callback.register "hello_world" hello_world
