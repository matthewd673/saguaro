%{
  let parse_error s =
    raise @@ Failure s;
  ;;
%}

%token <int> NUM
%token PROBLEM CNF END EOF

%start input
%type <(int * int) * Cnf.lit list list> input

%%

input: problem clause_list EOF  { $1, $2 }
;

problem: PROBLEM CNF NUM NUM { $3, $4 }
;

clause_list: (* Empty *)  { [] }
  | clause_list clause    { $2 :: $1 }
;

clause: indices END { $1 }
  | indices EOF     { $1 }
;

indices: (* Empty *)  { [] }
  | indices NUM       { $2 :: $1 }
;
%%
