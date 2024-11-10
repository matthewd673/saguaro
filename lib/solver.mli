type assignment = Cnf.var * bool

(** [string_of_assignment a]
    Returns: A string representation of assignment [a]. *)
val string_of_assignment : assignment -> string

(** [evaluate cnf trues]
    Evaluate formula [cnf] given the set of variables [trues] that are assigned
    true.
    Returns: [true] iff the formula is satisfied by this assignment. *)
val evaluate : Cnf.t -> Cnf.var list -> bool

(** [brute_force cnf]
    Determine, via a brute-force approach, if formula [cnf] is satisfiable.
    Returns: [true] iff the formula is satisfiable. *)
val brute_force : Cnf.t -> bool

(** [unit_prop cnf]
    Perform unit propagation on [cnf] until no units remain.
    Returns: A list of assignments and a simplified formula. *)
val unit_prop : Cnf.t -> assignment list * Cnf.t

(** [pure_lit_elim cnf]
    Perform pure literal elimination on [cnf] until no pure literals remain.
    Returns: A list of assignments and a simplified formula. *)
val pure_lit_elim : Cnf.t -> assignment list * Cnf.t
