type emit = { entry : Id.l; actual_fv : Id.t list }

let rec g env = function
  | KNormal.Unit -> ""
  | KNormal.Int(i) -> " Int " ^string_of_int(i)
  | KNormal.Float(d) -> string_of_float(d)
  | KNormal.And(x, y) ->(x ^ " && " ^ y)
  | KNormal.Or(x, y) -> (x ^ " || " ^ y)
  | KNormal.Neg(x) -> ("-" ^ x)
  | KNormal.Add(x, y) -> (x ^ " + " ^ y)
  | KNormal.Sub(x, y) -> (x ^ " - " ^ y)
  | KNormal.Mul(x, y) -> (x ^ " * " ^ y)
  | KNormal.Div(x, y) -> (x ^ " / " ^ y)
  | KNormal.FNeg(x) ->  ("-." ^ x)
  | KNormal.FAdd(x, y) -> (x ^ " +. " ^ y)
  | KNormal.FSub(x, y) -> (x ^ " -. " ^ y)
  | KNormal.FMul(x, y) -> (x ^ " *. " ^ y)
  | KNormal.FDiv(x, y) -> (x ^ " /. " ^ y)
  | KNormal.Let((x, t), e1, e2) -> (g env e1) ^ (g (M.add x t env) e2)
  | KNormal.IfEq(x, y, e1, e2) -> (g env e1) ^ " = " ^ (g env e2)
  | KNormal.IfLE(x, y, e1, e2) ->  (g env e1) ^ " <= " ^ (g env e2)
  | KNormal.While(e1, e2, e3, e4) ->  (g env e1) ^ e2 ^ (g env e3) ^ (g env e4)
  | KNormal.Var(x) -> x
  | KNormal.LetRec({ KNormal.name = (x, t); KNormal.args = yts; KNormal.body = e1 }, e2) -> x
  | KNormal.App(x, xs) -> x
  | KNormal.Tuple(xs) -> "Todo"
  | KNormal.LetTuple(xts, y, e) -> y ^ (g (M.add_list xts env) e)
  | KNormal.Get(x, y) -> (x ^ " " ^ y)
  | KNormal.Put(x, y, z) ->  (x ^ " " ^ y)
  | KNormal.ExtArray(x) -> "Todo1"
  | KNormal.ExtFunApp(x, ys) -> ("EXTFUNAPP " ^ x ^ "\n")


let f e = g M.empty e
