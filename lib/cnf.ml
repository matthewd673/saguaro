type var = string

type lit =
  | Var of var
  | Not of var

type clause = lit list

type t = clause list

let string_of_lit = function
  | Var var -> var
  | Not var -> Printf.sprintf "!%s" var
;;

let string_of_clause c =
  List.map string_of_lit c
  |> String.concat " || "
  |> Printf.sprintf "(%s)"
;;

let to_string cnf =
  List.map string_of_clause cnf
  |> String.concat " && "
;;
