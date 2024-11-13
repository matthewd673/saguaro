%{
  open Cnf

  let parse_error s =
    raise @@ Failure s;
  ;;
%}

%token <int> INDEX
%token PROBLEM CNF END NOT EOF

%start input
%type <(int * int) * Cnf.lit list list> input

%%

input: problem clause_list EOF  { $1, $2 }
;

problem: PROBLEM CNF INDEX INDEX { $3, $4 }
;

clause_list: (* Empty *)  { [] }
  | clause_list clause    { $2 :: $1 }
;

clause: indices END { $1 }
  | indices EOF     { $1 }
;

indices: (* Empty *)  { [] }
  | indices INDEX     { (Var (string_of_int $2)) :: $1 }
  | indices NOT INDEX { (Not (string_of_int $3)) :: $1 }
;
%%
