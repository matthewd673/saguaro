type token =
  | INDEX of (
# 9 "lib/cnf_file_parser.mly"
        int
# 6 "lib/cnf_file_parser.mli"
)
  | PROBLEM
  | CNF
  | END
  | NOT
  | EOF

val input :
  (Lexing.lexbuf  -> token) -> Lexing.lexbuf -> (int * int) * Cnf.lit list list
