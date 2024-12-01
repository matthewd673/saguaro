type unit_prop_result =
  | Ok of Cnf.t * Assignments.t
  | Conflict of Cnf.var

val evaluate : Cnf.t -> Cnf.var list -> bool

val unit_prop : Cnf.t -> Assignments.t -> unit_prop_result

val pure_lit_elim : Cnf.t -> Assignments.t -> Cnf.t * Assignments.t

val dpll : Cnf.t -> bool
