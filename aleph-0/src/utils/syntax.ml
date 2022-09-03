let counter = ref 0

type t =
  | Unit
  | Bool of bool
  | Int of int
  | Float of float
  | String of string
  | Not of t
  | And of t * t
  | Or of t * t
  | Neg of t
  | Add of t * t
  | Sub of t * t
  | Mul of t * t
  | Div of t * t
  | Eq of t * t
  | LE of t * t
  | If of t * t * t
  | While of t * t * t * t
  | Let of string * t * t
  | Var of string
  | LetRec of string * t list * t
  | App of t * t list
  | Tuple of t list
  | Array of t list
  | Get of string * t
  | Put of string * t * t * bool
  | Length of string
  | Stmts of t * t
  | Import of string

let rec getType = function
  | Unit -> Unit
  | Bool(b) -> Bool(b)
  | Int(i) -> Int(i)
  | Float(d) -> Float(d)
  | String(s) -> String(s)
  | Not(b) -> Bool(true)
  | And(x, y) -> Bool(true)
  | Or(x, y) -> Bool(true)
  | Eq(x, y) -> Bool(true)
  | LE(x, y) -> Bool(true)
  | Neg(x) -> let t = getType x in t
  | Add(x, y) -> 
    let t1 = getType x in 
    let t2 = getType y in 
    t1 (* should be unify with t2 *)
  | Sub(x, y) -> 
    let t1 = getType x in 
    let t2 = getType y in 
    t1 (* should be unify with t2 *)
  | Mul(x, y) -> 
    let t1 = getType x in 
    let t2 = getType y in 
    t1 (* should be unify with t2 *)
  | Div(x, y) ->
    let t1 = getType x in 
    let t2 = getType y in 
    t1 (* should be unify with t2 *)
  | Let(x, e1, Unit) ->
    let t1 = getType e1 in 
    t1
  | Let(x, e1, e2) ->
    let t1 = getType e1 in 
    t1
  | If(e1, e2, e3) ->
    let t1 = getType e2 in
    let t2 = getType e3 in  
    t1 (* should be unify with t2 *)
  | While(Unit, e2, e3, Unit) ->
    let t1 = getType e2 in
    let t2 = getType e3 in  
    t1 (* should be unify with t2 *)
  | While(e1, e2, e3, e4) -> 
    let t1 = getType e2 in
    let t2 = getType e3 in  
    t1 (* should be unify with t2 *)
  | Var(x) -> Unit (* todo defined *)
  | LetRec(name, args, e) ->
    let t1 = getType e in
    t1
  | App(x, xs) -> Unit
  | Tuple(xs) -> Tuple(xs)
  | Array(xs) -> Array(xs)
  | Get(x, y) -> Unit
  | Put(x, y, z, b) -> Unit
  | Length(x) -> Int(0)
  | Stmts(e1,e2) ->
    let t1 = getType e1 in
    let t2 = getType e2 in  
    t1 (* should be unify with t2 *)
  | Import(_) -> Unit

let primType = function
  | Unit -> "unit"
  | Bool(_) -> "bool"
  | Int(_) -> "int"
  | Float(_) -> "float"
  | String(_) -> "string"
  | _ -> assert false

let rec id_of_typ = function
  | Unit -> "u"
  | Bool(_) -> "b"
  | Int(_) -> "i"
  | Float(_) -> "d"
  | String(_) -> "s"
  | LetRec _ -> "f"
  | Tuple _ -> "t"
  | Array _ -> "a" 
  | _ -> assert false

let gentmp typ =
  incr counter;
  Printf.sprintf "T%s%d" (id_of_typ typ) !counter
