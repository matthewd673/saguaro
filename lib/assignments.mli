type t

val mem : Cnf.var -> t -> bool

val find : Cnf.var -> t -> bool

val add : Cnf.var -> bool -> t -> t

val cardinal : t -> int

val empty : t

val of_list : (Cnf.var * bool) list -> t

val to_string : t -> string
