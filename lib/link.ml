let incr2 x = x + 2

let () = Callback.register "incr2" incr2
