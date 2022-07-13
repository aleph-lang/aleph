open Parser
open Format

let print_token out = function
  | INT i ->
    fprintf out "%d\n" i
  | STRING s ->
    fprintf out "%s\n" s
  | _ ->
    ()


let lexbuf l =
  try
    let token = ref (Parser.main Lexer.token l) in
      printf "%a" (print_token) !token;
      while !token <> EOF do
        token := Parser.main Lexer.token l;
        printf "%a" (print_token) !token
      done
  with exn ->
      begin
        let curr = l.Lexing.lex_curr_p in
        let line = curr.Lexing.pos_lnum in
        let cnum = curr.Lexing.pos_cnum - curr.Lexing.pos_bol in
        let tok = Lexing.lexeme l in
        printf "Line : %d col : %d token : '%s'\n" line cnum tok;
        raise exn
      end

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