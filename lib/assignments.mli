(* module VarMap : Map.S with type key = Cnf.var *)

(* type t = bool VarMap.t *)

type t

(** [mem var assignments]
    Returns: [true] if [var] is assigned in [assignments]. *)
val mem : Cnf.var -> t -> bool

(** [find var assignments]
    Get the value of [var] in [assignments].
    Throws: [Not_found] if [var] is not assigned in [assignments]. *)
val find : Cnf.var -> t -> bool

(** [add var value assignments]
    Construct a new assignments map containing the bindings in [assignments]
    and a new binding of [var] to [value]. *)
val add : Cnf.var -> bool -> t -> t

(** [empty]
    Return: An empty assignments map. *)
val empty : t

(** [to_string assignments]
    Convert [assignments] to a string representation. *)
val to_string : t -> string
