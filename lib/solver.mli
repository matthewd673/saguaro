module VarMap : Map.S with type key = string
type assign_map = bool VarMap.t

(** [evaluate cnf trues]
    Evaluate formula [cnf] given the set of variables [trues] that are assigned
    true.
    Returns: [true] iff the formula is satisfied by this assignment. *)
val evaluate : Cnf.t -> Cnf.var list -> bool

(** [unit_prop cnf]
    Perform unit propagation on [cnf] until no units remain.
    Returns: A simplified formula and a list of assignments. *)
val unit_prop : Cnf.t -> Cnf.t * assign_map

(** [pure_lit_elim cnf]
    Perform pure literal elimination on [cnf] until no pure literals remain.
    Returns: A simplified formula and a list of assignments. *)
val pure_lit_elim : Cnf.t -> Cnf.t * assign_map

(** [dpll cnf]
    Determines, via the Davis-Putnam-Logemann-Loveland algorithm, if the formula
    [cnf] is satisfiable. *)
val dpll : Cnf.t -> bool

(** [brute_force cnf]
    Determine, via a brute-force approach, if formula [cnf] is satisfiable.
    Returns: [true] iff the formula is satisfiable. *)
val brute_force : Cnf.t -> bool
