open Cnf

let var_of_lit = function
  | Var var -> var
  | Not var -> var
;;

let in_list e =
  List.exists (fun x -> x = e)
;;

let distinct l =
  let rec aux acc = function
    | [] -> acc (* Don't bother reversing because order doesn't matter *)
    | h :: t ->
        aux (if in_list h acc then acc else (h :: acc)) t
  in
  aux [] l
;;

let collect_vars cnf =
  distinct (List.map var_of_lit (List.flatten cnf))
;;

let bool_of_lit trues = function
  | Not var -> not (in_list var trues)
  | Var var -> in_list var trues

let evaluate_clause trues c = (* If a var is not in [trues] it is false *)
  List.map (fun l -> bool_of_lit trues l) c
  |> List.fold_left (||) false
;;

let evaluate cnf trues =
  List.map (evaluate_clause trues) cnf
  |> List.fold_left (&&) true
;;

let brute_force cnf =
  let rec aux cnf trues = function
    | [] -> evaluate cnf trues
    | h :: t ->
        (aux cnf (h :: trues) t) ||
        (aux cnf trues t)
  in
  let vars = collect_vars cnf in
  aux cnf [] vars
;;
