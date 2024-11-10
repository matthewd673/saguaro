open OUnit2
open Saguaro

let assert_mset_equal a b =
  assert_equal (List.length a) (List.length b);
  a
  |> List.map (fun x -> (List.mem x b))
  |> List.fold_left (&&) true
  |> assert_equal true
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

let test_unit_prop cnf (exp_assign, exp_cnf) _ =
  let (act_assign, act_cnf) = Solver.unit_prop cnf in
  assert_mset_equal exp_assign act_assign;
  assert_equal exp_cnf act_cnf;
;;

let make_unit_prop_test (cnf, exp_assign, exp_cnf) =
  let cnf_str = Cnf.to_string cnf in
  let exp_assign_str = String.concat "; "
    (List.map (fun (v, b) -> Printf.sprintf "%s=%b" v b) exp_assign) in
  let exp_cnf_str = Cnf.to_string exp_cnf in
  (Printf.sprintf "Unit prop transforms %s -> %s with [%s]"
    cnf_str exp_cnf_str exp_assign_str) >::
    (fun ctx -> test_unit_prop cnf (exp_assign, exp_cnf) ctx)
;;

let unit_prop_tests =
  List.map make_unit_prop_test
  [[], [], [];
   [[Var "a"]], ["a", true], [];
   [[Not "a"]], ["a", false], [];
   [[Var "a"]; [Var "b"]], ["a", true; "b", true], [];
   [[Var "a"]; [Not "a"]], ["a", true], [[]];
   [[Var "a"]; [Var "a"; Var "b"]], ["a", true], [];
   [[Var "a"]; [Not "a"; Var "b"]], ["a", true; "b", true], [];
   [[Var "a"]; [Not "a"; Var "b"; Var "c"]], ["a", true], [[Var "b"; Var "c"]];
   [[Var "a"]; [Not "a"]], ["a", true], [[]];
   [[Var "a"; Var "b"]; [Var "a"]; [Not "b"]],
    ["a", true; "b", false], [];
   [[Var "a"]; [Not "b"]; [Var "c"; Not "d"]],
    ["a", true; "b", false],
    [[Var "c"; Not "d"]];
   ]
;;

let test_pure_lit_elim cnf (exp_assign, exp_cnf) _ =
  let (act_assign, act_cnf) = Solver.pure_lit_elim cnf in
  assert_mset_equal exp_assign act_assign;
  assert_equal exp_cnf act_cnf;
;;

let make_pure_lit_elim_test (cnf, exp_assign, exp_cnf) =
  let cnf_str = Cnf.to_string cnf in
  let exp_assign_str = String.concat "; "
    (List.map (fun (v, b) -> Printf.sprintf "%s=%b" v b) exp_assign) in
  let exp_cnf_str = Cnf.to_string exp_cnf in
  (Printf.sprintf "Pure lit elim transforms %s -> %s with [%s]"
    cnf_str exp_cnf_str exp_assign_str) >::
    (fun ctx -> test_pure_lit_elim cnf (exp_assign, exp_cnf) ctx)
;;

let pure_lit_elim_tests =
  List.map make_pure_lit_elim_test
  [[[Var "a"]; [Var "a"; Not "b"]; [Var "b"]],
    ["a", true],
    [[]; [Not "b"]; [Var "b"]];
   [[Var "a"]; [Not "a"]], [], [[Var "a"]; [Not "a"]];
   [[Var "a"; Not "b"]; [Not "a"]], ["b", false], [[Var "a"]; [Not "a"]];
   ]
;;

let suite =
  "Saguaro tests" >:::
    eval_tests
    @ unit_prop_tests
    @ pure_lit_elim_tests
;;

let () = run_test_tt_main suite
