open Saguaro

let () =
  try
    let filename = Array.get Sys.argv 1 in
    let file_channel = In_channel.open_bin filename in
    let lexbuf = Lexing.from_channel file_channel in
    let ((n_var, n_clause), clauses) =
      Cnf_file_parser.input Cnf_file_lexer.token lexbuf
    in
    (* TODO: Validate that the clauses match the problem definition *)
    Printf.printf "Problem def: vars=%d, clauses=%d\n" n_var n_clause;

    print_endline "Solving (DPLL)...";
    print_endline (if Solver.dpll clauses then "SAT" else "UNSAT");
  with Failure err -> begin
    Printf.printf "Failure: %s\n" err;
    exit 1;
  end

;;
