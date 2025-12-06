let euclidean_div a b =
  let r = a mod b in
  if r >= 0 then (a / b, r) else (1 - (a / b), b + r)

let run (input : String.t) : int =
  let clicks_list =
    String.split_on_char '\n' input
    |> List.map (fun line ->
        let direction = String.get line 0 in
        let clicks =
          int_of_string (String.sub line 1 ((max 2 @@ String.length line) - 1))
        in
        if direction == 'R' then clicks else -clicks)
  in
  let (_, positions) : _ * int List.t =
    clicks_list
    |> List.fold_left_map
         (fun acc click ->
           let _q, r = euclidean_div (acc + click) 100 in
           (r, r))
         50
  in
  positions |> List.fold_left (fun acc p -> if p == 0 then acc + 1 else acc) 0

let () =
  let t = Sys.time () in
  let output = run Sys.argv.(1) in
  let run_time_ms = 1000.0 *. (Sys.time () -. t) in
  Format.printf "_duration:%F\n%d\n" run_time_ms output
