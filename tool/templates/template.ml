let run (_input: String.t): int = 0

let () =
  let t = Sys.time () in
  let output = run Sys.argv.(1) in
  let run_time_ms = 1000.0 *. (Sys.time () -. t) in
  Format.printf "_duration:%F\n%d\n" run_time_ms output
