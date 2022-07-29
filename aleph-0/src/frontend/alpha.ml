(* rename identifiers to make them unique (alpha-conversion) *)

open KNormal

let find x env = try M.find x env with Not_found -> x

let rec g env = function
  | Unit -> Unit
  | Int(i) -> Int(i)
  | Float(d) -> Float(d)
  | Neg(x) -> Neg(find x env)
  | And(x, y) -> And(find x env, find y env)
  | Or(x, y) -> Or(find x env, find y env)
  | Add(x, y) -> Add(find x env, find y env)
  | Sub(x, y) -> Sub(find x env, find y env)
  | Mul(x, y) -> Mul(find x env, find y env)
  | Div(x, y) -> Div(find x env, find y env)
  | FNeg(x) -> FNeg(find x env)
  | FAdd(x, y) -> FAdd(find x env, find y env)
  | FSub(x, y) -> FSub(find x env, find y env)
  | FMul(x, y) -> FMul(find x env, find y env)
  | FDiv(x, y) -> FDiv(find x env, find y env)
  | IfEq(x, y, e1, e2) -> IfEq(find x env, find y env, g env e1, g env e2)
  | IfLE(x, y, e1, e2) -> IfLE(find x env, find y env, g env e1, g env e2)
  | While(e1, e2, e3, e4) -> While(g env e1, find e2 env, g env e3, g env e4)
  | Let((x, t), e1, e2) ->
      let x' = Id.genid x in
      Let((x', t), g env e1, g (M.add x x' env) e2)
  | Var(x) -> Var(find x env)
  | LetRec({ name = (x, t); args = yts; body = e1 }, e2) ->
      let env = M.add x (Id.genid x) env in
      let ys = List.map fst yts in
      let env' = M.add_list2 ys (List.map Id.genid ys) env in
      LetRec({ name = (find x env, t);
               args = List.map (fun (y, t) -> (find y env', t)) yts;
               body = g env' e1 },
             g env e2)
  | App(x, ys) -> App(find x env, List.map (fun y -> find y env) ys)
  | Tuple(xs) -> Tuple(List.map (fun x -> find x env) xs)
  | LetTuple(xts, y, e) ->
      let xs = List.map fst xts in
      let env' = M.add_list2 xs (List.map Id.genid xs) env in
      LetTuple(List.map (fun (x, t) -> (find x env', t)) xts,
               find y env,
               g env' e)
  | Get(x, y) -> Get(find x env, find y env)
  | Put(x, y, z) -> Put(find x env, find y env, find z env)
  | ExtArray(x) -> ExtArray(x)
  | ExtFunApp(x, ys) -> ExtFunApp(x, List.map (fun y -> find y env) ys)

let rec printKnormal = function
  | Unit -> Printf.printf "Unit"
  | Int(i) -> Printf.printf "Int %d " i
  | Float(f) -> Printf.printf "Float %f " f
  | Neg(x) -> Printf.printf "- %s " x
  | And(x, y) -> Printf.printf "And %s %s " x y
  | Or(x, y) -> Printf.printf "Or %s %s " x y
  | Add(x, y) -> Printf.printf "+ %s %s " x y
  | Sub(x, y) -> Printf.printf "- %s %s " x y
  | Mul(x, y) -> Printf.printf "* %s %s " x y
  | Div(x, y) -> Printf.printf "/ %s %s " x y
  | FNeg(x) -> Printf.printf "-. %s " x
  | FAdd(x, y) -> Printf.printf "+. %s %s " x y
  | FSub(x, y) -> Printf.printf "-. %s %s " x y
  | FMul(x, y) -> Printf.printf "*. %s %s " x y
  | FDiv(x, y) -> Printf.printf "/. %s %s " x y
  | IfEq(x, y, e1, e2) -> Printf.printf "Neg %s" x
  | IfLE(x, y, e1, e2) -> Printf.printf "Neg %s" x
  | While(e1, e2, e3, e4) -> Printf.printf "While "; printKnormal e1; Printf.printf "%s" e2; printKnormal e3; printKnormal e4
  | Let((x, t), e1, e2) ->
      Printf.printf "PKno Let %s = " x; printKnormal e1; printKnormal e2; Printf.printf "\n"
  | Var(x) -> Printf.printf "Neg %s" x
  | LetRec({ name = (x, t); args = yts; body = e1 }, e2) ->
      Printf.printf "Neg %s" x
  | App(x, ys) -> Printf.printf "PKno App %s " x
  | Tuple(xs) -> Printf.printf "Tuple"
  | LetTuple(xts, y, e) ->
      Printf.printf "LetTuple "
  | Get(x, y) -> Printf.printf "Get %s " x
  | Put(x, y, z) -> Printf.printf "Put %s " x
  | ExtArray(x) -> Printf.printf "ExtArray %s " x
  | ExtFunApp(x, ys) -> Printf.printf "ExtFunApp %s \n" x

let f e = 
  printKnormal e;
  g M.empty e
