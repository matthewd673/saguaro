open Cnf

exception Contradiction

type assignment = Cnf.var * bool

let string_of_assignment (v, b) =
  Printf.sprintf "%s=%b" v b
;;

let var_of_lit = function
  | Var var -> var
  | Not var -> var
;;

let inverse_of_lit = function
  | Var var -> Not var
  | Not var -> Var var
;;

let sat_of_lit = function
  | Var _ -> true
  | Not _ -> false
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
  cnf
  |> List.filter (fun c -> not (List.is_empty c))
  |> List.map (evaluate_clause trues)
  |> List.fold_left (&&) true
;;

let unit_prop_on_clause lit = function
  (* Remove unit clause *)
  | [l] when l = lit -> []
  (* Inverse unit clause is contradiction *)
  | [l] when l = inverse_of_lit lit -> raise Contradiction
  (* Clauses that contain lit will be satisfied if lit is *)
  | c when in_list lit c -> []
  (* Clauses that contain !lit can never be satisfied by that *)
  | c -> List.filter (fun x -> x <> inverse_of_lit lit) c
;;

let unit_prop_lit lit cnf =
  sat_of_lit lit, List.map (unit_prop_on_clause lit) cnf
;;

let unit_prop cnf =
  let rec aux assignments cnf =
    let units = cnf
    |> List.filter (fun c -> List.length c = 1)
    |> List.map (fun u -> List.hd u)
    in

    if List.is_empty units
      then assignments, cnf
      else
        let u = List.hd units in
        let (u_a, cnf_res) = unit_prop_lit u cnf in
        aux ((var_of_lit u, u_a) :: assignments) cnf_res
  in
  aux [] cnf
;;

let find_pure_lits cnf =
  let lits = List.flatten cnf in
  lits
  |> List.filter (fun x -> not (in_list (inverse_of_lit x) lits))
  |> distinct
;;

let pure_lit_elim cnf =
  let pure_lits = find_pure_lits cnf in
  List.map (fun l -> var_of_lit l, sat_of_lit l) pure_lits,
  List.map (List.filter (fun x -> not (in_list x pure_lits))) cnf
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
