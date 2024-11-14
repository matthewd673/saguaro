type var = int

type lit = int

type clause = lit list

type t = clause list

let string_of_lit =
  string_of_int
;;

let string_of_clause c =
  List.map string_of_lit c
  |> String.concat " | "
  |> Printf.sprintf "(%s)"
;;

let to_string cnf =
  List.map string_of_clause cnf
  |> String.concat " & "
;;
