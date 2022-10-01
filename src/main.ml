let confFile = ref ""
let outputDir = ref "out"
let outputFile = ref "output.out"
let useStdout = ref "false"

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
        | "gen" -> !Filter.gen ast outchan ((List.length l)> 2 && bool_of_string(List.nth l 2)); ast
        | _ -> ast
      in compute b lexbuf t outchan
    end

let lexbuf outchan l =
  if !confFile = ""
  then begin
    if (!useStdout == "false") then Printf.printf "Configuration file : Default - conf/ale2ocaml.conf (Ale -> Ocaml)\n" else ();
    confFile := "conf/ale2ocaml.conf"
  end else begin
    if (!useStdout == "false") then Printf.printf "Configuration file : %s\n" !confFile else ();
  end;
  let lines = read_file !confFile in
  let _ = compute lines l Unit outchan in ()

let file f =
  let inchan = open_in (f) in
  let outchan = (if (not(bool_of_string(!useStdout))) then (open_out(!outputDir ^ "/"^ !outputFile)) else (stdout)) in
  try
    lexbuf outchan (Lexing.from_channel inchan);
    close_in inchan;
    close_out outchan;
  with e -> (close_in inchan; close_out outchan; raise e)

let computeLines l =
  let inchan = String.concat "\n" l in
  let outchan = (if (not(bool_of_string(!useStdout))) then (open_out(!outputDir ^ "/"^ !outputFile)) else (stdout)) in
  try
    lexbuf outchan (Lexing.from_string inchan);
    close_out outchan;
  with e -> (close_out outchan; raise e)

let () =
  let files = ref [] in
  Arg.parse
    [("-conf", Arg.String(fun s -> confFile := s), "Compiler configuration file");
     ("-oDir", Arg.String(fun s -> outputDir := s), "output directory");
     ("-o", Arg.String(fun s -> outputFile := s), "output file");
     ("-stdout", Arg.String(fun s -> useStdout := s), "use stdout");
    ]
    (fun s -> files := !files @ [s])
    ("Aleph compiler\n" ^
     Printf.sprintf "usage: %s filenames" Sys.argv.(0));
  if (!files = []) then (
    (* STDIN *)
    let lines = ref [] in
    try
        while true do
            lines := !lines @ [read_line ()];
        done
    with
        End_of_file -> ignore (computeLines !lines)
  ) else (List.iter
    (fun f -> ignore (file f))
    !files
  )
