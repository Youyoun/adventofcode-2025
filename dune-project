(lang dune 3.17)

(name aoc)

; (generate_opam_files true)

(package
 (name aoc)
 (depends 
  (ocaml-variants (= 5.4.0+options))
  ocaml-option-flambda
  ocaml-option-no-flat-float-array

  (iter (= 1.8))
  (base (>= 0.17))
  ))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/reference/dune-project/index.html
