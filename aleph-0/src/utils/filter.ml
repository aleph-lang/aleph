let parser : (unit -> Syntax.t) ref = ref (fun _ -> assert false)
let transform : (Syntax.t -> Syntax.t) ref = ref (fun _ -> assert false)
let gen : (Syntax.t -> out_channel -> unit) ref = ref (fun _ -> assert false)
