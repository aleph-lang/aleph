(* Call from dynlink *)
let () =
  Filter.transform := (fun e -> Printf.printf "Ident2\n"; e)