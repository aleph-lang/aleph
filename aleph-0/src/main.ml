let confFile = ref ""

let read_file filename = 
let lines = ref [] in
let chan = open_in filename in
try
  while true; do
    lines := input_line chan :: !lines
  done; !lines
with End_of_file ->
  close_in chan;
  List.rev !lines ;;

let lexbuf outchan l =
  Id.counter := 0;
  let process = 
  (if !confFile = ""
  then begin
    Printf.printf "Configuraiton file : Default (Al0 -> Ocaml)\n";
    Ocaml.f
  end else begin
    Printf.printf "Configuraiton file : %s\n" !confFile;
    let lines = read_file !confFile in
    Printf.printf "%s\n" (String.concat " " lines);
    Ocaml.f
  end) in
  let res = process (Parser.exp Lexer.token l) in
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
    [("-conf", Arg.String(fun s -> confFile := s), "Compiler configuration file")]
    (fun s -> files := !files @ [s])
    ("Aleph-0 compiler\n" ^
     Printf.sprintf "usage: %s filenames" Sys.argv.(0));
  List.iter
    (fun f -> ignore (file f))
    !files
