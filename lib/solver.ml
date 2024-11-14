module VarMap = Map.Make(Int);;
type assign_map = bool VarMap.t

let var_of_lit =
  abs
;;

let sat_of_lit lit =
  lit > 0
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
  cnf
  |> List.flatten
  |> List.map var_of_lit
;;

let bool_of_lit trues lit =
  if lit < 0
    then not @@ List.mem (var_of_lit lit) trues
    else List.mem (var_of_lit lit) trues
;;

let evaluate_clause trues = (* If a var is not in [trues] it is false *)
  let rec aux = function
    | [] -> false
    | l :: t ->
        if bool_of_lit trues l
          then true
          else aux t
  in
  aux
;;

let evaluate cnf trues =
  let rec aux = function
    | [] -> true
    | c :: t ->
        if not @@ evaluate_clause trues c
          then false
          else aux t
  in
  aux cnf
;;

let is_lit_unit lit = function
  | [l] when l = lit -> true
  | _ -> false
;;

let unit_prop_lit lit cnf =
  let transform = cnf
  (* Remove unit clauses of lit *)
  |> List.filter (fun c -> not @@ is_lit_unit lit c)
  (* Remove clauses containing the literal *)
  |> List.filter (fun c -> not @@ List.mem lit c)
  (* Remove the inverse polarity of the literal from clauses it appears in.
     If the inverse polarity is in a unit clause, the clause will become empty
     which is UNSAT. *)
  |> List.map (List.filter (fun l -> l <> -lit))
  in
  transform, sat_of_lit lit
;;

let unit_prop cnf =
  let rec aux cnf assigns =
    (* Search for units each time since this may cascade *)
    let units = cnf
    |> List.filter (fun c -> List.length c = 1)
    |> List.map (fun u -> List.hd u)
    in

    if List.is_empty units
      then cnf, assigns
      else
        let u = List.hd units in
        let (transform, u_a) = unit_prop_lit u cnf in
        aux transform (VarMap.add (var_of_lit u) u_a assigns)
  in
  aux cnf VarMap.empty
;;

let find_pure_lits cnf =
  let lits = List.flatten cnf in
  lits
  |> List.filter (fun x -> not @@ List.mem (-x) lits)
;;

let assign_pure_lits pure_lits =
  let rec aux map = function
    | [] -> map
    | h :: t ->
        aux (VarMap.add (var_of_lit h) (sat_of_lit h) map) t
  in
  aux VarMap.empty pure_lits
;;

let pure_lit_elim cnf =
  let pure_lits = find_pure_lits cnf in

  List.map (List.filter (fun x -> not @@ List.mem x pure_lits)) cnf,
  assign_pure_lits pure_lits
;;

let rec dpll cnf =
  let (cnf, up_assign) = unit_prop cnf in
  let (cnf, ple_assign) = pure_lit_elim cnf in

  let assign = VarMap.union
    (fun _ _ _ -> raise @@ Failure "Invalid assignments")
    up_assign
    ple_assign
  in
  match cnf with
  | [] -> true (* No clauses is SAT *)
  | cnf when List.mem [] cnf -> false (* An empty clause is always UNSAT *)
  | cnf ->
      let v = List.find
        (fun x -> not @@ VarMap.mem x assign)
        (collect_vars cnf)
      in
      dpll ([v] :: cnf) || dpll ([-v] :: cnf)
;;

let brute_force cnf =
  let rec aux cnf trues = function
    | [] -> evaluate cnf trues
    | h :: t ->
        (aux cnf (h :: trues) t) ||
        (aux cnf trues t)
  in
  let vars = distinct @@ collect_vars cnf in
  aux cnf [] vars
;;
