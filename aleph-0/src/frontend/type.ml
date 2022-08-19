type t =
  | Unit
  | Bool
  | Int
  | Float
  | Fun of t list * t (* arguments are uncurried *)
  | Tuple of t list
  | Array of t list
  | Var of t option ref

let gentyp () = Var(ref None)
