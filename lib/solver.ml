open Cnf

module VarSet = Set.Make(String)

let var_of_term = function
  | Not var -> var
  | Var var -> var
;;

let collect_vars cnf =
  VarSet.of_list (List.map var_of_term (List.flatten cnf))
;;

let solve cnf =
  let vars = collect_vars cnf in
  Printf.printf "Variables (%d): " (VarSet.cardinal vars);
  List.iter (Printf.printf "%s ") (VarSet.to_list vars);
  print_endline "";
;;
