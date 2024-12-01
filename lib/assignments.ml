module VarMap = Map.Make(Int);;

type t = bool VarMap.t

let mem = VarMap.mem

let find = VarMap.find

let add = VarMap.add

let cardinal = VarMap.cardinal

let empty = VarMap.empty

let of_list = VarMap.of_list

let to_string assignments =
  assignments
  |> VarMap.bindings
  |> List.map (fun (k, v) -> Printf.sprintf "%s%d" (if v then "" else "-") k)
  |> String.concat " "
;;
