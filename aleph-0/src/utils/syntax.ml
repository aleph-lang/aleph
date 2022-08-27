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

let rec id_of_typ = function
  | Unit -> "u"
  | Bool(_) -> "b"
  | Int(_) -> "i"
  | Float(_) -> "d"
  | LetRec _ -> "f"
  | Tuple _ -> "t"
  | Array _ -> "a" 
  | _ -> assert false

let gentmp typ =
  incr counter;
  Printf.sprintf "T%s%d" (id_of_typ typ) !counter
