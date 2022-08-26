(* Call from dynlink *)
let () =
  Filter.parse := (fun l -> Parser.exp Lexer.token l)