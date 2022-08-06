open Syntax

let rec g env = function
  | Unit -> "()"
  | Bool(b) -> " Bool " ^string_of_bool(b)
  | Int(i) -> " Int " ^string_of_int(i)
  | Float(d) -> string_of_float(d)
  | Not(x) -> "not " ^ (g env x)
  | And(x, y) ->(g env x) ^ " && " ^ (g env y)
  | Or(x, y) -> (g env x) ^ " || " ^ (g env y)
  | Eq(x, y) -> (g env x) ^ " = " ^ (g env y)
  | LE(x, y) -> (g env x) ^ " <= " ^ (g env y)
  | Neg(x) -> "-" ^ (g env x)
  | Add(x, y) -> (g env x) ^ " + " ^ (g env y)
  | Sub(x, y) -> (g env x) ^ " - " ^ (g env y)
  | Mul(x, y) -> (g env x) ^ " * " ^ (g env y)
  | Div(x, y) -> (g env x) ^ " / " ^ (g env y)
  | FNeg(x) ->  "-." ^ (g env x)
  | FAdd(x, y) -> (g env x) ^ " +. " ^ (g env y)
  | FSub(x, y) -> (g env x) ^ " -. " ^ (g env y)
  | FMul(x, y) -> (g env x) ^ " *. " ^ (g env y)
  | FDiv(x, y) -> (g env x) ^ " /. " ^ (g env y)
  | Let((x, t), e1, e2) -> " let " ^ x ^ " = " ^ (g env e1) ^" in "^ (g (M.add x t env) e2)
  | If(e1, e2, e3) -> (g env e1) ^ " then " ^ (g env e2) ^ " else " ^ (g env e3)
  | While(e1, e2, e3, e4) ->  (g env e1) ^ (g env e2) ^ (g env e3) ^ (g env e4)
  | Var(x) -> x
  | LetRec({ name = (x, t); args = yts; body = e1 }, e2) -> x ^ (g env e1) ^ (g env e2)
  | App(x, xs) -> (g env x) ^ "(" ^ (String.concat " " (List.map (g env) xs)) ^ ")\n"
  | Return(e) -> "return " ^ (g env e)
  | Tuple(xs) -> (String.concat " " (List.map (g env) xs))
  | LetTuple(xts, y, e) -> (g env y) ^ (g (M.add_list xts env) e)
  | Get(x, y) -> (g env x) ^ " " ^ (g env y)
  | Put(x, y, z) ->  (g env x) ^ " " ^ (g env y)
  | Array(x, y) -> (g env x) ^ " " ^ (g env y)


let f e = g M.empty e
