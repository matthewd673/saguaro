open Saguaro
open Cnf

let () =
  print_endline "saguaro";

  let cnf = [[Var "x1"]; [Not "x2"]] in
  Solver.solve cnf
;;
