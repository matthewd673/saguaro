open OUnit2
open Saguaro
open Solver

let test_eval cnf trues sat _ =
  assert_equal sat (Solver.evaluate cnf trues)
;;

let make_eval_test (cnf, trues, sat) =
  let cnf_str = Cnf.to_string cnf in
  let trues_str = String.concat "; " (List.map string_of_int trues) in
  (Printf.sprintf "Eval %s when [%s] true -> %b" cnf_str trues_str sat) >::
    (fun ctx -> test_eval cnf trues sat ctx)
;;

let eval_tests =
  List.map make_eval_test
  [[], [], true;
   [[1]], [1], true;
   [[1]], [], false;
   [[1; -1]], [], true;
   [[1; -1]], [1], true;
   [[1; -1]; []], [1], false;
   [[1]; [-1]], [], false;
   [[1]; [-1]], [1], false;
   [[1]; [2]], [1], false;
   [[1]; [2]], [2], false;
   [[1]; [2]], [1; 2], true;
   [[1; -2]; [2]], [1; 2], true;
   [[1]; [-2]], [1], true;
   ]
;;

let test_unit_prop cnf exp_result _ =
  assert_equal exp_result (Solver.unit_prop cnf Assignments.empty)
;;

let make_unit_prop_test (cnf, exp_result) =
  let cnf_str = Cnf.to_string cnf in
  begin match exp_result with
  | Conflict v -> Printf.sprintf "Unit prop on %s -> conflict %d" cnf_str v
  | Ok (exp_cnf, exp_assign) ->
      let exp_cnf_str = Cnf.to_string exp_cnf in
      let exp_assign_str = Assignments.to_string exp_assign in
      Printf.sprintf "Unit prop on %s -> %s with %s" cnf_str exp_cnf_str exp_assign_str
  end >:: (fun ctx -> test_unit_prop cnf exp_result ctx)
;;

let unit_prop_tests =
  List.map make_unit_prop_test
  [[], Ok ([], Assignments.empty);
   [[1]], Ok ([], Assignments.of_list [1, true]);
   [[-1]], Ok ([], Assignments.of_list [1, false]);
   [[1]; [1]], Ok ([], Assignments.of_list [1, true; 1, true]);
   [[1]; [1; 2]], Ok ([], Assignments.of_list [1, true]);
   [[1]; [-1; 2]], Ok ([], Assignments.of_list [1, true; 2, true]);
   [[1]; [-1; 2; 3]], Ok ([[2; 3]], Assignments.of_list [1, true]);
   [[1; 2]; [1]; [-2]], Ok ([], Assignments.of_list [1, true; 2, false]);
   [[1]; [-2]; [3; -4]], Ok ([[3; -4]], Assignments.of_list [1, true; 2, false]);
   [[1]; [-1]], Conflict 1;
   [[1]; [-1; 2]; [-2]], Conflict 2;
   ]
;;

let test_pure_lit_elim cnf exp_cnf exp_assign _ =
  let act_cnf, act_assign = Solver.pure_lit_elim cnf Assignments.empty in
  assert_equal exp_cnf act_cnf ~printer:Cnf.to_string ~msg:"Cnf does not match";
  assert_equal exp_assign act_assign ~printer:Assignments.to_string ~msg:"Assignment map does not match";
;;

let make_pure_lit_elim_test (cnf, exp_cnf, exp_assign) =
  let cnf_str = Cnf.to_string cnf in
  let exp_cnf_str = Cnf.to_string exp_cnf in
  let exp_assign_str = Assignments.to_string exp_assign in
  (Printf.sprintf "Pure lit elim transforms %s -> %s with %s" cnf_str exp_cnf_str exp_assign_str) >::
    (fun ctx -> test_pure_lit_elim cnf exp_cnf exp_assign ctx)
;;

let pure_lit_elim_tests =
  List.map make_pure_lit_elim_test
  [[[1]; [1; -2]; [2]], [[2]], Assignments.of_list [1, true];
   [[1]; [-1]], [[1]; [-1]], Assignments.empty;
   (* TODO: Optimize for this case, can get [2, false; 1, true] after unit prop *)
   [[1; -2]; [-1]], [[-1]], Assignments.of_list [2, false];
    [[1; 2]; [3; 4]], [], Assignments.of_list [1, true; 2, true; 3, true; 4, true];
   ]
;;

let suite =
  "Saguaro tests" >:::
    eval_tests
    @ unit_prop_tests
    @ pure_lit_elim_tests
;;

let () = run_test_tt_main suite
