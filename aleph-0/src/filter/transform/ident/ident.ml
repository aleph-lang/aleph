(* Call from dynlink *)
let () =
  Filter.transform := (fun e -> Printf.printf "Ident1\n"; e)