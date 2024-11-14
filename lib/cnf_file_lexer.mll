{
  open Cnf_file_parser
}

rule token = parse
  | [' ' '\t' '\n']                   { token lexbuf }
  | 'c'                               { comment lexbuf }
  | 'p'                               { PROBLEM }
  | "cnf"                             { CNF }
  | '0'                               { END }
  | '-'? ['1'-'9'] ['0'-'9']* as ind  { NUM (int_of_string ind) }
  | eof                               { EOF }

and comment = parse
  | '\n'  { token lexbuf }
  | _     { comment lexbuf }
  | eof   { EOF }
