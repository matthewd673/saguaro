type var = int

type lit = int

type clause = lit list

type t = clause list

(** [to_string cnf]
    Convert the formula [cnf] to a string representation. *)
val to_string : t -> string
