type emit = { entry : Id.l; actual_fv : Id.t list }
type t =
  | Unit
  | Int of int
  | Float of float
  | Neg of Id.t
  | And of Id.t * Id.t
  | Or of Id.t * Id.t
  | Add of Id.t * Id.t
  | Sub of Id.t * Id.t
  | Mul of Id.t * Id.t
  | Div of Id.t * Id.t
  | FNeg of Id.t
  | FAdd of Id.t * Id.t
  | FSub of Id.t * Id.t
  | FMul of Id.t * Id.t
  | FDiv of Id.t * Id.t
  | IfEq of Id.t * Id.t * t * t
  | IfLE of Id.t * Id.t * t * t
  | While of t * Id.t * t * t
  | Let of (Id.t * Type.t) * t * t
  | Var of Id.t
  | MakeCls of (Id.t * Type.t) * emit * t
  | AppCls of Id.t * Id.t list
  | AppDir of Id.l * Id.t list
  | Tuple of Id.t list
  | LetTuple of (Id.t * Type.t) list * Id.t * t
  | Get of Id.t * Id.t
  | Put of Id.t * Id.t * Id.t
  | ExtArray of Id.l
type fundef = { name : Id.l * Type.t;
                args : (Id.t * Type.t) list;
                formal_fv : (Id.t * Type.t) list;
                body : t }
type prog = Prog of fundef list * t

let rec fv = function
  | Unit | Int(_) | Float(_) | ExtArray(_) -> S.empty
  | Neg(x) | FNeg(x) -> S.singleton x
  | Add(x, y) | Sub(x, y) | Mul(x, y) | Div(x, y) | FAdd(x, y) | FSub(x, y) | FMul(x, y) | FDiv(x, y) | Get(x, y) | And(x,y) | Or(x,y) -> S.of_list [x; y]
  | IfEq(x, y, e1, e2)| IfLE(x, y, e1, e2) -> S.add x (S.add y (S.union (fv e1) (fv e2)))
  | Let((x, t), e1, e2) -> S.union (fv e1) (S.remove x (fv e2))
  | Var(x) -> S.singleton x
  | MakeCls((x, t), { entry = l; actual_fv = ys }, e) -> S.remove x (S.union (S.of_list ys) (fv e))
  | AppCls(x, ys) -> S.of_list (x :: ys)
  | AppDir(_, xs) | Tuple(xs) -> S.of_list xs
  | LetTuple(xts, y, e) -> S.add y (S.diff (fv e) (S.of_list (List.map fst xts)))
  | Put(x, y, z) -> S.of_list [x; y; z]

let toplevel : fundef list ref = ref []

let rec g l env known = function
  | KNormal.Unit -> ""::l
  | KNormal.Int(i) -> string_of_int(i)::l
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
  | KNormal.App(f, xs) -> f::l
  (*| KNormal.LetRec({ KNormal.name = (x, t); KNormal.args = yts; KNormal.body = e1 }, e2) ->
      let toplevel_backup = !toplevel in
      let env' = M.add x t env in
      let known' = S.add x known in
      let e1' = g (M.add_list yts env') known' e1 in
      let zs = S.diff (fv e1') (S.of_list (List.map fst yts)) in
      let known', e1' =
        if S.is_empty zs then known', e1' else
        (Format.eprintf "free variable(s) %s found in function %s@." (Id.pp_list (S.elements zs)) x;
         Format.eprintf "function %s cannot be directly applied in fact@." x;
         toplevel := toplevel_backup;
         let e1' = g (M.add_list yts env') known e1 in
         known, e1') in
      let zs = S.elements (S.diff (fv e1') (S.add x (S.of_list (List.map fst yts)))) in
      let zts = List.map (fun z -> (z, M.find z env')) zs in
      toplevel := { name = (Id.L(x), t); args = yts; formal_fv = zts; body = e1' } :: !toplevel;
      let e2' = g env' known' e2 in
      if S.mem x (fv e2') then
        MakeCls((x, t), { entry = Id.L(x); actual_fv = zs }, e2')
      else
        (Format.eprintf "eliminating emit(s) %s@." x;
         e2')
 
  *)
  | KNormal.App(x, ys) when S.mem x known ->
      Format.eprintf "directly applying %s@." x;
      x::l
  | KNormal.App(f, xs) -> f::l
  | KNormal.Tuple(xs) -> "Todo"::l
  | KNormal.LetTuple(xts, y, e) -> y::(g l (M.add_list xts env) known e) @ l
  | KNormal.Get(x, y) -> (x ^ " " ^ y)::l
  | KNormal.Put(x, y, z) ->  (x ^ " " ^ y)::l
  | KNormal.ExtArray(x) -> "Todo1"::l
  | KNormal.ExtFunApp(x, ys) -> (x ^ "\n")::l


let f e =
  toplevel := [];
  let l = g [] M.empty S.empty e in
  let cl2s cl = String.concat " " cl in
  (cl2s l)
