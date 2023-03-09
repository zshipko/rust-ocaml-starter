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

(* The rest of the file demonstrates the use of Lwt, which can be bound to a
Future in Rust. *)

(* Here we define the polling logic on the OCaml side.
This code is adapted from the Lwt_main.run implementation here:
https://github.com/ocsigen/lwt/blob/9943ba77a5508feaea5e1fb60b011db4179f9c61/src/unix/lwt_main.ml#L27 *)

(* DO NOT SUBMIT: Do these sequences or the code that uses them do anything? *)
let enter_iter_hooks = Lwt_sequence.create ()
let leave_iter_hooks = Lwt_sequence.create ()
let yielded = Lwt_sequence.create ()

let lwt_poll p =
  let rec run_loop () =
    (* Fulfill paused promises now. *)
    Lwt.wakeup_paused ();
    match Lwt.poll p with
    | Some x -> Some x
    | None ->
      (* Call enter hooks. *)
      Lwt_sequence.iter_l (fun f -> f ()) enter_iter_hooks;

      (* Do the main loop call. *)
      let should_block_waiting_for_io =
        Lwt.paused_count () = 0 && Lwt_sequence.is_empty yielded in
      Lwt_engine.iter should_block_waiting_for_io;

      (* Fulfill paused promises again. *)
      Lwt.wakeup_paused ();

      (* Fulfill yield promises. *)
      if not (Lwt_sequence.is_empty yielded) then begin
        let tmp = Lwt_sequence.create () in
        Lwt_sequence.transfer_r yielded tmp;
        Lwt_sequence.iter_l (fun resolver -> Lwt.wakeup resolver ()) tmp
      end;

      (* Call leave hooks. *)
      Lwt_sequence.iter_l (fun f -> f ()) leave_iter_hooks;

      (* Unlike in Lwt_main.run, we return after a single attempt. *)
      None
  in

  run_loop ()

let () = Callback.register "lwt_poll" lwt_poll

(* Here we define a function that uses Lwt. *)
let fetch_example_dot_com () =
  let open Lwt.Infix in
  let open Cohttp_lwt_unix in
  Client.get (Uri.of_string "http://example.com/") >>= fun (_, body) ->
  print_endline "Fetching http://example.com/";
  body |> Cohttp_lwt.Body.to_string

let () = Callback.register "fetch_example_dot_com" fetch_example_dot_com