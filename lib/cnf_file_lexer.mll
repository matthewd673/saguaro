{
  open Cnf_file_parser
}

rule token = parse
  | [' ' '\t' '\n']             { token lexbuf }
  | 'c'                         { comment lexbuf }
  | 'p'                         { PROBLEM }
  | "cnf"                       { CNF }
  | '0'                         { END }
  | ['0'-'9']+ as ind           { INDEX (int_of_string ind) }
  | '-'                         { NOT }
  | eof                         { EOF }

and comment = parse
  | '\n'  { token lexbuf }
  | _     { comment lexbuf }
  | eof   { EOF }
