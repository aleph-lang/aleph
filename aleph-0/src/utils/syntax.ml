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
  | Let of (Id.t * Type.t) * t * t
  | Var of Id.t
  | LetRec of Id.t * t list * t
  | App of t * t list
  | Tuple of t list
  | LetTuple of (Id.t * Type.t) list * t * t
  | Array of t list
  | Get of Id.t * t
  | Put of Id.t * t * t * bool
  | Length of Id.t
