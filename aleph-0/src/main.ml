let lexbuf outchan l =
  Id.counter := 0;
  let res = Ocaml.f (Parser.exp Lexer.token l) in
  Printf.fprintf outchan "%s\n" res

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
    []
    (fun s -> files := !files @ [s])
    ("Aleph-0 compiler\n" ^
     Printf.sprintf "usage: %s filenames" Sys.argv.(0));
  List.iter
    (fun f -> ignore (file f))
    !files
