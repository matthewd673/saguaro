open OUnit2
open Saguaro
open Solver

let string_of_assign_map assign_map =
  assign_map
  |> VarMap.to_list
  |> List.map (fun (v, b) -> Printf.sprintf "%d=%b" v b)
  |> String.concat "; "
  |> Printf.sprintf "[%s]"
;;

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

let test_unit_prop cnf (exp_cnf, exp_assign) _ =
  let (act_cnf, act_assign) = Solver.unit_prop cnf in
  assert_equal exp_assign act_assign;
  assert_equal exp_cnf act_cnf;
;;

let make_unit_prop_test (cnf, exp_cnf, exp_assign) =
  let cnf_str = Cnf.to_string cnf in
  let exp_assign_str = string_of_assign_map exp_assign in
  let exp_cnf_str = Cnf.to_string exp_cnf in
  (Printf.sprintf "Unit prop transforms %s -> %s with [%s]"
    cnf_str exp_cnf_str exp_assign_str) >::
    (fun ctx -> test_unit_prop cnf (exp_cnf, exp_assign) ctx)
;;

let unit_prop_tests =
  List.map make_unit_prop_test
  [[], [], VarMap.empty;
   [[1]], [], VarMap.of_list [1, true];
   [[-1]], [], VarMap.of_list [1, false];
   [[1]; [1]], [], VarMap.of_list [1, true; 1, true];
   [[1]; [-1]], [[]], VarMap.of_list [1, true];
   [[1]; [1; 2]], [], VarMap.of_list [1, true];
   [[1]; [-1; 2]], [], VarMap.of_list [1, true; 2, true];
   [[1]; [-1; 2; 3]], [[2; 3]], VarMap.of_list [1, true];
   [[1]; [-1]], [[]], VarMap.of_list [1, true];
   [[1; 2]; [1]; [-2]], [], VarMap.of_list [1, true; 2, false];
   [[1]; [-2]; [3; -4]], [[3; -4]], VarMap.of_list [1, true; 2, false];
   ]
;;

let test_pure_lit_elim cnf (exp_cnf, exp_assign) _ =
  let (act_cnf, act_assign) = Solver.pure_lit_elim cnf in
  assert_equal true (exp_assign = act_assign);
  assert_equal exp_cnf act_cnf;
;;

let make_pure_lit_elim_test (cnf, exp_cnf, exp_assign) =
  let cnf_str = Cnf.to_string cnf in
  let exp_assign_str = string_of_assign_map exp_assign in
  let exp_cnf_str = Cnf.to_string exp_cnf in
  (Printf.sprintf "Pure lit elim transforms %s -> %s with [%s]"
    cnf_str exp_cnf_str exp_assign_str) >::
    (fun ctx -> test_pure_lit_elim cnf (exp_cnf, exp_assign) ctx)
;;

let pure_lit_elim_tests =
  List.map make_pure_lit_elim_test
  [[[1]; [1; -2]; [2]], [[]; [-2]; [2]], VarMap.of_list [1, true];
   [[1]; [-1]], [[1]; [-1]], VarMap.empty;
   [[1; -2]; [-1]], [[1]; [-1]], VarMap.of_list [2, false];
   ]
;;

let suite =
  "Saguaro tests" >:::
    eval_tests
    @ unit_prop_tests
    @ pure_lit_elim_tests
;;

let () = run_test_tt_main suite
