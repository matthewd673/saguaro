open Cnf

type assignment = Cnf.var * bool

let string_of_assignment (v, b) =
  Printf.sprintf "%s=%b" v b
;;

let var_of_lit = function
  | Var var -> var
  | Not var -> var
;;

let inv_of_lit = function
  | Var var -> Not var
  | Not var -> Var var
;;

let sat_of_lit = function
  | Var _ -> true
  | Not _ -> false
;;

let distinct l =
  let rec aux acc = function
    | [] -> acc (* Don't bother reversing because order doesn't matter *)
    | h :: t ->
        aux (if List.mem h acc then acc else (h :: acc)) t
  in
  aux [] l
;;

let collect_vars cnf =
  distinct (List.map var_of_lit (List.flatten cnf))
;;

let bool_of_lit trues = function
  | Not var -> not (List.mem var trues)
  | Var var -> List.mem var trues

let evaluate_clause trues c = (* If a var is not in [trues] it is false *)
  List.map (fun l -> bool_of_lit trues l) c
  |> List.fold_left (||) false
;;

let evaluate cnf trues =
  cnf
  |> List.map (evaluate_clause trues)
  |> List.fold_left (&&) true
;;

let is_lit_unit lit = function
  | [l] when l = lit -> true
  | _ -> false
;;

let unit_prop_lit lit cnf =
  let transform = cnf
  (* Remove unit clauses of lit *)
  |> List.filter (fun c -> not (is_lit_unit lit c))
  (* Remove clauses containing the literal *)
  |> List.filter (fun c -> not (List.mem lit c))
  (* Remove the inverse polarity of the literal from clauses it appears in.
     If the inverse polarity is in a unit clause, the clause will become empty
     which is UNSAT. *)
  |> List.map (List.filter (fun l -> l <> inv_of_lit lit))
  in

  sat_of_lit lit, transform
;;

let unit_prop cnf =
  let rec aux assignments cnf =
    (* Search for units each time since this may cascade *)
    let units = cnf
    |> List.filter (fun c -> List.length c = 1)
    |> List.map (fun u -> List.hd u)
    in

    if List.is_empty units
      then assignments, cnf
      else
        let u = List.hd units in
        let (u_a, transform) = unit_prop_lit u cnf in
        aux ((var_of_lit u, u_a) :: assignments) transform
  in
  aux [] cnf
;;

let find_pure_lits cnf =
  let lits = List.flatten cnf in
  lits
  |> List.filter (fun x -> not (List.mem (inv_of_lit x) lits))
  |> distinct (* Pure lits will likely occur multiple times *)
;;

let pure_lit_elim cnf =
  let pure_lits = find_pure_lits cnf in
  List.map (fun l -> var_of_lit l, sat_of_lit l) pure_lits,
  List.map (List.filter (fun x -> not (List.mem x pure_lits))) cnf
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
