open OUnit2
open Saguaro

let test_eval cnf trues sat _ =
  assert_equal (Solver.evaluate cnf trues) sat
;;

let suite =
  "Evaluate tests" >:::
    begin List.map
      begin fun (cnf, trues, sat) ->
        let cnf_str = Cnf.to_string cnf in
        let trues_str = String.concat ", " trues in
        (Printf.sprintf "%s when %s -> %b" cnf_str trues_str sat) >::
          (fun ctx -> test_eval cnf trues sat ctx)
      end
      [[], [], true;
       [[Var "a"]], ["a"], true;
       [[Var "a"]], [], false;
       [[Var "a"; Not "a"]], [], true;
       [[Var "a"; Not "a"]], ["a"], true;
       [[Var "a"]; [Not "a"]], [], false;
       [[Var "a"]; [Not "a"]], ["a"], false;
       [[Var "a"]; [Var "b"]], ["a"], false;
       [[Var "a"]; [Var "b"]], ["b"], false;
       [[Var "a"]; [Var "b"]], ["a"; "b"], true;
       [[Var "a"; Not "b"]; [Var "b"]], ["a"; "b"], true;
       [[Var "a"]; [Not "b"]], ["a"], true;
       ]
    end
;;

let () = run_test_tt_main suite
