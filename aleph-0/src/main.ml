let limit = ref 1000

let rec iter n e =
  Format.eprintf "iteration %d@." n;
  if n = 0 then e else
  let e' = Elim.f (ConstFold.f (Inline.f (Assoc.f (Beta.f e)))) in
  if e = e' then e else
  iter (n - 1) e'

let lexbuf outchan l =
  Id.counter := 0;
  Typing.extenv := M.empty;
  let res = Emit.f
      (iter !limit
        (Alpha.f
          (KNormal.f
            (Typing.f
              (Parser.exp Lexer.token l))))) in
  Printf.fprintf outchan "%s\n" res

let string s = lexbuf stdout (Lexing.from_string s)

let file f =
  let inchan = open_in (f) in
  let outchan = open_out("out/al0c.out") in
  try
    lexbuf outchan (Lexing.from_channel inchan);
    close_in inchan;
    close_out outchan;
  with e -> (close_in inchan; close_out outchan; raise e)

let () =
  let files = ref [] in
  Arg.parse
    [("-inline", Arg.Int(fun i -> Inline.threshold := i), "maximum size of functions inlined");
     ("-iter", Arg.Int(fun i -> limit := i), "maximum number of optimizations iterated")]
    (fun s -> files := !files @ [s])
    ("Aleph-0 compiler\n" ^
     Printf.sprintf "usage: %s [-inline m] [-iter n] ...filenames without \".ml\"..." Sys.argv.(0));
  List.iter
    (fun f -> ignore (file f))
    !files
