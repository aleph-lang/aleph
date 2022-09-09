let confFile = ref ""
let outputDir = ref "out"
let outputFile = ref "output.out"

let read_file filename = 
  let lines = ref [] in
  let chan = open_in filename in
  try
    while true; do
      lines := input_line chan :: !lines
    done; !lines
  with End_of_file ->
    close_in chan;
    List.rev !lines

let rec compute list lexbuf ast outchan = match list with
  | [] -> ast
  | h::b -> begin let l = Str.split (Str.regexp " ") h in
      Dynlink.loadfile (List.nth l 0);
      let t = match (List.nth l 1) with
        | "parse" -> !Filter.parse lexbuf
        | "transform" -> !Filter.transform ast
        | "gen" -> !Filter.gen ast outchan; ast
        | _ -> ast
      in compute b lexbuf t outchan
    end

let lexbuf outchan l =
  if !confFile = ""
  then begin
    Printf.printf "Configuration file : Default - conf/ale2ocaml.conf (Ale -> Ocaml)\n";
    confFile := "conf/ale2ocaml.conf"
  end else begin
    Printf.printf "Configuration file : %s\n" !confFile;
  end;
  let lines = read_file !confFile in
  let _ = compute lines l Unit outchan in ()

let file f =
  let inchan = open_in (f) in
  let path = !outputDir ^ "/"^ !outputFile in
  let outchan = open_out(path) in
  try
    lexbuf outchan (Lexing.from_channel inchan);
    close_in inchan;
    close_out outchan;
  with e -> (close_in inchan; close_out outchan; raise e)

let () =
  let files = ref [] in
  Arg.parse
    [("-conf", Arg.String(fun s -> confFile := s), "Compiler configuration file");
     ("-oDir", Arg.String(fun s -> outputDir := s), "output directory");
     ("-o", Arg.String(fun s -> outputFile := s), "output file")
    ]
    (fun s -> files := !files @ [s])
    ("Aleph compiler\n" ^
     Printf.sprintf "usage: %s filenames" Sys.argv.(0));
  List.iter
    (fun f -> ignore (file f))
    !files
