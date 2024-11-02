type var = string

type term =
  | Not of var
  | Var of var

type clause = term list

type t = clause list
