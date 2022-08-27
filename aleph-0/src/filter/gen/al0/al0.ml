open Syntax

let rec gen = function
  | Unit -> ""
  | Bool(b) -> string_of_bool(b)
  | Int(i) -> string_of_int(i)
  | Float(d) -> string_of_float(d)
  | String(s) -> s
  | Not(x) -> "!(" ^ (gen x) ^ ")"
  | And(x, y) -> (gen x) ^ " & " ^ (gen y)
  | Or(x, y) -> (gen x) ^ " | " ^ (gen y)
  | Eq(x, y) -> (gen x) ^ " = " ^ (gen y)
  | LE(x, y) ->(gen x) ^ " <= " ^ (gen y)
  | Neg(x) -> "-" ^ (gen x)
  | Add(x, y) -> (gen x) ^ " + " ^ (gen y)
  | Sub(x, y) -> (gen x) ^ " - " ^ (gen y)
  | Mul(x, y) -> (gen x) ^ " * " ^ (gen y)
  | Div(x, y) -> (gen x) ^ " / " ^ (gen y)
  | Let(x, e1, Unit) -> x ^ " = " ^ (gen e1)
  | Let(x, e1, e2) -> x ^ " = " ^ (gen e1) ^";\n"^ (gen e2)
  | If(e1, e2, e3) -> "Al0" ^"if " ^ (gen e1) ^ " then " ^ (gen e2) ^ " else " ^ (gen e3)
  | While(Unit, e2, e3, Unit) ->   "Al0" ^ "while "^ (gen e2) ^ " do  " ^ (gen e3) ^ ";" ^ " done"
  | While(e1, e2, e3, e4) ->  "Al0" ^ (gen e1) ^ "while "^ (gen e2) ^ " do  " ^ (gen e3) ^ ";" ^ (gen e4) ^ " done"
  | Var(x) -> x
  | LetRec(name, args, e) -> "Al0" ^"let rec " ^ name ^ " " ^ (String.concat " " (List.map gen args)) ^ " = " ^ (gen e)
  | App(Var("print"), xs) -> "print(" ^ (String.concat " " (List.map (gen) xs)) ^ ")"
  | App(x, xs) -> "Al0" ^(gen x) ^ "(" ^ (String.concat " " (List.map (gen) xs)) ^ ")\n"
  | Tuple(xs) -> "Al0" ^(String.concat " " (List.map (gen) xs))
  | Array(xs) -> "Al0" ^"ARRAY : " ^ (String.concat " " (List.map gen xs))
  | Get(x, y) -> "Al0" ^"Get : " ^ x ^ "[" ^ (gen y) ^ "]"
  | Put(x, y, z, b) -> "Al0" ^"Put : insert ? "^ string_of_bool(b) ^ ", "^ x ^ "[" ^ (gen y) ^ "] = " ^ (gen z)
  | Length(x) -> "Al0" ^x ^ ".length()"

(* Call from dynlink *)
let () =
  Filter.gen := (fun e outchan -> Printf.fprintf outchan "%s\n" (gen e))