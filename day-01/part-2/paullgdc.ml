
let new_pos original incr =
  if incr >= 0
    then 
      let new_pos = original + incr in
    new_pos / 100, new_pos mod 100
  else
    let reverted_new_pos =  (((100 - original) mod 100) - incr) in
    reverted_new_pos / 100, (100 - (reverted_new_pos mod 100)) mod 100

let run (input: String.t) : int = 
  let (_final_pos, zero_counts) = String.split_on_char '\n' input
  |> List.fold_left (fun (pos, zero_counts) line -> 
    let direction = String.get line 0 in
    let clicks_str = int_of_string (String.sub line 1 ((max 2 @@ String.length line) - 1)) in
    let clicks = (if direction == 'R' then clicks_str else - clicks_str) in
      let (zeros_crossed, pos) = new_pos pos clicks in
      (pos, zero_counts + zeros_crossed)
    ) (50, 0) in
  zero_counts

let () =
  let t = Sys.time () in
  let output = run Sys.argv.(1) in
  let run_time_ms = 1000.0 *. (Sys.time () -. t) in
  Format.printf "_duration:%F\n%d\n" run_time_ms output