(* Wrap a simple function. *)

let hello_world () = "Hello, world!"

let () = Callback.register "hello_world" hello_world

(* Wrap a function that operates on a variant type `t`.
   Note `t` itself does not need to be wrapped here - it only needs to be
   described in the Rust code. *)

type t = A | B of int

let maybe_inc (t : t): t =
  match t with
    | A -> A
    | B x -> B (x + 1)

let () = Callback.register "maybe_inc" maybe_inc

(* Wrap a struct and also demonstrate using modules. *)

module MyStruct = struct
  type t = {a: float; b: int}

  let inc_both (t:t): t = {a = t.a +. 1.0; b = t.b + 1}
end

(* The interface to Rust has no notion of modules, so all exported functions
   live in the toplevel namespace.
   Thus you may want to prefix functions in modules with the module name, as
   done here. *)
let () = Callback.register "mystruct_inc_both" MyStruct.inc_both
