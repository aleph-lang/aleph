let gen : (Syntax.t -> out_channel -> unit) ref = ref (fun _ -> assert false)

let process : (Syntax.t -> Syntax.t) ref = ref (fun _ -> assert false)