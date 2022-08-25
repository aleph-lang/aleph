open Syntax

let rec g env = function
  | Unit -> ""
  | Bool(b) -> string_of_bool(b)
  | Int(i) -> string_of_int(i)
  | Float(d) -> string_of_float(d)
  | String(s) -> s
  | Not(x) -> "!(" ^ (g env x) ^ ")"
  | And(x, y) -> (g env x) ^ " & " ^ (g env y)
  | Or(x, y) -> (g env x) ^ " | " ^ (g env y)
  | Eq(x, y) -> (g env x) ^ " = " ^ (g env y)
  | LE(x, y) ->(g env x) ^ " <= " ^ (g env y)
  | Neg(x) -> "-" ^ (g env x)
  | Add(x, y) -> (g env x) ^ " + " ^ (g env y)
  | Sub(x, y) -> (g env x) ^ " - " ^ (g env y)
  | Mul(x, y) -> (g env x) ^ " * " ^ (g env y)
  | Div(x, y) -> (g env x) ^ " / " ^ (g env y)
  | Let((x, t), e1, Unit) -> x ^ " = " ^ (g env e1)
  | Let((x, t), e1, e2) -> x ^ " = " ^ (g env e1) ^";\n"^ (g (M.add x t env) e2)
  | If(e1, e2, e3) -> "Al0" ^"if " ^ (g env e1) ^ " then " ^ (g env e2) ^ " else " ^ (g env e3)
  | While(Unit, e2, e3, Unit) ->   "Al0" ^ "while "^ (g env e2) ^ " do  " ^ (g env e3) ^ ";" ^ " done"
  | While(e1, e2, e3, e4) ->  "Al0" ^ (g env e1) ^ "while "^ (g env e2) ^ " do  " ^ (g env e3) ^ ";" ^ (g env e4) ^ " done"
  | Var(x) -> x
  | LetRec(name, args, e) -> "Al0" ^"let rec " ^ name ^ " " ^ (String.concat " " (List.map (g env) args)) ^ " = " ^ (g env e)
  | App(Var("print"), xs) -> "print(" ^ (String.concat " " (List.map (g env) xs)) ^ ")"
  | App(x, xs) -> "Al0" ^(g env x) ^ "(" ^ (String.concat " " (List.map (g env) xs)) ^ ")\n"
  | Tuple(xs) -> "Al0" ^(String.concat " " (List.map (g env) xs))
  | LetTuple(xts, y, e) -> "Al0" ^(g env y) ^ (g (M.add_list xts env) e)
  | Array(xs) -> "Al0" ^"ARRAY : " ^ (String.concat " " (List.map (g env) xs))
  | Get(x, y) -> "Al0" ^"Get : " ^ x ^ "[" ^ (g env y) ^ "]"
  | Put(x, y, z, b) -> "Al0" ^"Put : insert ? "^ string_of_bool(b) ^ ", "^ x ^ "[" ^ (g env y) ^ "] = " ^ (g env z)
  | Length(x) -> "Al0" ^x ^ ".length()"


let gen e = g M.empty e

(* Call from dynlink *)
let () =
  Filter.gen := (fun e outchan -> Printf.fprintf outchan "%s\n" (gen e))