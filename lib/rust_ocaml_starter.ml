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
  type t = {a: int; b: float}

  let inc_both (t:t): t = {a = t.a + 1; b = t.b +. 1.0}
end

(* The interface to Rust has no notion of modules, so all exported functions live
in the toplevel namespace.
Thus you may want to prefix functions in modules with the module name, as done
here. *)
let () = Callback.register "mystruct_inc_both" MyStruct.inc_both

(* Demonstrate using tuples. *)

let tuple_inc_both ((a, b): int * float): (int * float) = (a + 1, b +. 1.0)
let () = Callback.register "tuple_inc_both" tuple_inc_both

(* Wrap an opaque type, and also demonstrate using modules. *)

module Opaque = struct
  (* Let's use an OCaml Out_channel as an example of an OCaml type which can't be
  naturally converted to a Rust type.
  Thus we must treat this type as opaque. *)
  type t = {oc: Out_channel.t}

  let create (): t = {oc = Out_channel.stdout}

  (* We're returning a string here so we can print something later, but it is
  not necessary for correctness. *)
  let use (t: t): string = Out_channel.flush t.oc; "Opaque.use called"
end

let () = Callback.register "opaque_create" Opaque.create
let () = Callback.register "opaque_use" Opaque.use

(* !!! Everything below here is broken !!! *)

module BrokenStruct = struct
  type t = {a: float; b: float}

  let inc_both (t:t): t = {a = t.a +. 1.0; b = t.b +. 1.0}
end

let () = Callback.register "brokenstruct_inc_both" BrokenStruct.inc_both

