open Saguaro
open Cnf

let () =
  let cnf = [[Var "a"]; [Not "a"; Var "b"]; [Not "b"; Var "c"]] in
  Printf.printf "Expression: %s\n" (Cnf.to_string cnf);

  let sat = Solver.dpll cnf in
  print_endline (if sat then "SAT" else "UNSAT");
;;
