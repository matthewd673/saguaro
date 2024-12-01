module VarSet = Set.Make(Int);;

let var_of_lit =
  abs
;;

let sat_of_lit lit =
  lit > 0
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

type unit_prop_result =
  | Ok of Cnf.t * Assignments.t
  | Conflict of Cnf.var

let rec unit_prop cnf assign =
  (* Search for units each time since this may cascade *)
  let units = cnf
  |> List.filter (fun c -> List.length c = 1)
  |> List.map (fun u -> List.hd u)
  in

  if List.is_empty units
    then Ok (cnf, assign)
    else
      let u = List.hd units in
      (* Check for a conflict unit before propagating *)
      if List.exists (fun l -> l = -u) units
        then Conflict (var_of_lit u)
        else begin
          let (transform, u_a) = unit_prop_lit u cnf in
          unit_prop transform (Assignments.add (var_of_lit u) u_a assign)
        end
;;

let find_pure_lits cnf =
  let rec aux lit_set = function
    | [] -> lit_set
    | h :: t -> aux (VarSet.add h lit_set) t
  in
  let lits = List.flatten cnf in
  let lit_set = aux VarSet.empty lits in
  let pures = VarSet.filter (fun l -> not @@ VarSet.mem (-l) lit_set) lit_set in
  VarSet.to_list pures
;;

let pure_lit_elim cnf assign =
  let rec aux cnf assign = function
    | [] -> cnf, assign
    | h :: t ->
        let (transform, pl_assign) = unit_prop_lit h cnf in
        aux transform (Assignments.add (var_of_lit h) pl_assign assign) t
  in
  aux cnf assign (find_pure_lits cnf)
;;

let find_unassigned cnf assign =
  let rec aux = function
    | [] -> raise Not_found
    | h :: t -> match List.find_opt (fun l -> not @@ Assignments.mem (var_of_lit l) assign) h with
        | Some u -> var_of_lit u
        | None -> aux t
  in
  aux cnf
;;

let dpll cnf =
  let rec aux cnf assign =
    match unit_prop cnf assign with
    | Conflict _ -> false
    | Ok (cnf, assign) -> begin
      match cnf with
      | [] -> true (* No clauses is SAT *)
      | cnf when List.mem [] cnf -> false (* An empty clause is always UNSAT *)
      | cnf ->
          let v = find_unassigned cnf assign in
          aux ([v] :: cnf) assign || aux ([-v] :: cnf) assign
    end
  in
  let cnf, assign = pure_lit_elim cnf Assignments.empty in
  aux cnf assign
;;
