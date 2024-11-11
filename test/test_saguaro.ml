open OUnit2
open Saguaro
open Solver

let string_of_assign_map assign_map =
  assign_map
  |> VarMap.to_list
  |> List.map (fun (v, b) -> Printf.sprintf "%s=%b" v b)
  |> String.concat "; "
  |> Printf.sprintf "[%s]"
;;

let test_eval cnf trues sat _ =
  assert_equal sat (Solver.evaluate cnf trues)
;;

let make_eval_test (cnf, trues, sat) =
  let cnf_str = Cnf.to_string cnf in
  let trues_str = String.concat "; " trues in
  (Printf.sprintf "Eval %s when [%s] true -> %b" cnf_str trues_str sat) >::
    (fun ctx -> test_eval cnf trues sat ctx)
;;

let eval_tests =
  List.map make_eval_test
  [[], [], true;
   [[Var "a"]], ["a"], true;
   [[Var "a"]], [], false;
   [[Var "a"; Not "a"]], [], true;
   [[Var "a"; Not "a"]], ["a"], true;
   [[Var "a"; Not "a"]; []], ["a"], false;
   [[Var "a"]; [Not "a"]], [], false;
   [[Var "a"]; [Not "a"]], ["a"], false;
   [[Var "a"]; [Var "b"]], ["a"], false;
   [[Var "a"]; [Var "b"]], ["b"], false;
   [[Var "a"]; [Var "b"]], ["a"; "b"], true;
   [[Var "a"; Not "b"]; [Var "b"]], ["a"; "b"], true;
   [[Var "a"]; [Not "b"]], ["a"], true;
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
   [[Var "a"]], [], VarMap.of_list ["a", true];
   [[Not "a"]], [], VarMap.of_list ["a", false];
   [[Var "a"]; [Var "b"]], [], VarMap.of_list ["a", true; "b", true];
   [[Var "a"]; [Not "a"]], [[]], VarMap.of_list ["a", true];
   [[Var "a"]; [Var "a"; Var "b"]], [], VarMap.of_list ["a", true];
   [[Var "a"]; [Not "a"; Var "b"]], [], VarMap.of_list ["a", true; "b", true];
   [[Var "a"]; [Not "a"; Var "b"; Var "c"]],
    [[Var "b"; Var "c"]],
    VarMap.of_list ["a", true];
   [[Var "a"]; [Not "a"]], [[]], VarMap.of_list ["a", true];
   [[Var "a"; Var "b"]; [Var "a"]; [Not "b"]],
    [],
    VarMap.of_list ["a", true; "b", false];
   [[Var "a"]; [Not "b"]; [Var "c"; Not "d"]],
    [[Var "c"; Not "d"]],
    VarMap.of_list ["a", true; "b", false];
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
  [[[Var "a"]; [Var "a"; Not "b"]; [Var "b"]],
    [[]; [Not "b"]; [Var "b"]],
    VarMap.of_list ["a", true];
   [[Var "a"]; [Not "a"]], [[Var "a"]; [Not "a"]], VarMap.empty;
   [[Var "a"; Not "b"]; [Not "a"]],
    [[Var "a"]; [Not "a"]],
    VarMap.of_list ["b", false];
   ]
;;

let suite =
  "Saguaro tests" >:::
    eval_tests
    @ unit_prop_tests
    @ pure_lit_elim_tests
;;

let () = run_test_tt_main suite
