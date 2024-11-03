open Saguaro
open Cnf

let () =
  let cnf = [[Var "x1"; Not "x1"]; [Not "x2"]] in
  Printf.printf "Expression: %s\n" (Cnf.to_string cnf);

  let sat = Solver.brute_force cnf in
  print_endline (if sat then "SAT" else "UNSAT");
;;
