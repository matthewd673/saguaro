(** [evaluate cnf trues]
    Evaluate formula [cnf] given the set of variables [trues] that are assigned
    true.
    Returns: [true] iff the formula is satisfied by this assignment. *)
val evaluate : Cnf.t -> Cnf.var list -> bool

(** [brute_force cnf]
    Determine, via a brute-force approach, if formula [cnf] is satisfiable.
    Returns: [true] iff the formula is satisfiable. *)
val brute_force : Cnf.t -> bool
