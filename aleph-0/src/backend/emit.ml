type emit = { entry : Id.l; actual_fv : Id.t list }

let rec g l env known = function
  | KNormal.Unit -> ""::l
  | KNormal.Int(i) -> [" Int "] @string_of_int(i)::l
  | KNormal.Float(d) -> string_of_float(d)::l
  | KNormal.And(x, y) ->(x ^ " && " ^ y)::l
  | KNormal.Or(x, y) -> (x ^ " || " ^ y)::l
  | KNormal.Neg(x) -> ("-" ^ x)::l
  | KNormal.Add(x, y) -> (x ^ " + " ^ y)::l
  | KNormal.Sub(x, y) -> (x ^ " - " ^ y)::l
  | KNormal.Mul(x, y) -> (x ^ " * " ^ y)::l
  | KNormal.Div(x, y) -> (x ^ " / " ^ y)::l
  | KNormal.FNeg(x) ->  ("-." ^ x)::l
  | KNormal.FAdd(x, y) -> (x ^ " +. " ^ y)::l
  | KNormal.FSub(x, y) -> (x ^ " -. " ^ y)::l
  | KNormal.FMul(x, y) -> (x ^ " *. " ^ y)::l
  | KNormal.FDiv(x, y) -> (x ^ " /. " ^ y)::l
  | KNormal.Let((x, t), e1, e2) -> (g l env known e1) @ (g l (M.add x t env) known e2)  @ l
  | KNormal.IfEq(x, y, e1, e2) -> (g l env known e1) @ [" = "] @ (g l env known e2) @ l
  | KNormal.IfLE(x, y, e1, e2) ->  (g l env known e1) @ [" <= "] @ (g l env known e2) @ l
  | KNormal.While(e1, e2, e3, e4) ->  (g l env known e1) @ [e2] @ (g l env known e3) @ (g l env known e4) @ l
  | KNormal.Var(x) -> x::l
  | KNormal.LetRec({ KNormal.name = (x, t); KNormal.args = yts; KNormal.body = e1 }, e2) -> x::l
  | KNormal.App(x, ys) when S.mem x known ->
      Format.eprintf "directly applying %s@." x;
      x::l
  | KNormal.App(x, xs) -> x::l
  | KNormal.Tuple(xs) -> "Todo"::l
  | KNormal.LetTuple(xts, y, e) -> y::(g l (M.add_list xts env) known e) @ l
  | KNormal.Get(x, y) -> (x ^ " " ^ y)::l
  | KNormal.Put(x, y, z) ->  (x ^ " " ^ y)::l
  | KNormal.ExtArray(x) -> "Todo1"::l
  | KNormal.ExtFunApp(x, ys) -> ("EXTFUNAPP " ^ x ^ "\n")::l


let f e =
  let l = g [] M.empty S.empty e in
  let cl2s cl = String.concat " " cl in
  (cl2s l)
