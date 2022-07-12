open Parser
open Format

let print_token out = function
  | INT i ->
    fprintf out "INTEGER %d" i
  | _ ->
    fprintf out "EOF"


let lexbuf l =
  let token = ref (Parser.main Lexer.token l) in
    printf "%a\n" (print_token) !token;
    while !token <> EOF do
      token := Parser.main Lexer.token l;
      printf "%a\n" (print_token) !token
    done

let file f =
  let inchan = open_in (f) in
  try
    lexbuf (Lexing.from_channel inchan);
    close_in inchan;
  with e -> (close_in inchan; raise e)

let () = (* main *)
  let files = ref [] in
    Arg.parse
    []
    (fun s -> files := !files @ [s])
    ("Aleph-0 compiler\n" ^
     Printf.sprintf "usage: %s ...filenames..." Sys.argv.(0));
  List.iter
    (fun f -> ignore (file f))
    !files