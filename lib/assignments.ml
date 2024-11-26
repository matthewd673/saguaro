module VarMap = Map.Make(Int);;

type t = bool VarMap.t

let mem = VarMap.mem

let find = VarMap.find

let add = VarMap.add

let empty = VarMap.empty

let to_string assignments =
  assignments
  |> VarMap.bindings
  |> List.map (fun (k, v) -> Printf.sprintf "%s%d" (if v then "-" else "") k)
  |> String.concat " "
;;
