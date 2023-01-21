# ARC computation files

These files are intended to be run by the `arc` binary built from the `crates/arc` directory.


### TODO
- Operators and conjugates as "function" calls?
  - slippery slope...

- Comment printing to re-wrap lines in each paragrph

- Showing which side terms came from in the output
  - Defaulting to L/R as a signifier would work but may be noisy.
  - ANSI color would work for terminal output but not for piped...

- A way of doing conjugate products in a single expression (they show up a lot):
  ```
  // Bare
  (B + E) (B + E)!

  // With assignment
  m = B + E
  m m!

  // Would be nice (but notation needs to be worked out)
  `B + E`!
  ```

- Output reprs
  - Default / full   -> just with default repr methods
  - Simple form rep  -> simple set of alpha values present
  - Shorthand        -> pthq / BTAE
  - Vector Calc      -> Need a better solution than the python one...
  - Norm&versor      -> TODO: needs work done to compute this for MultiVectors (or AR?)
  - LaTeX            -> probably need several formats depending on notation

- Simple function/macro definition for cases like this where there is a common sequence of operations
  to be performed.
  - No control flow
  - Dynamic typing as with the main evaluator
  - Input lines should print as normal and the `Value` of calling a function should be the final line
    - i.e. no explicit return syntax (as with rust macros)
    - Maybe want a directive to show macro inputs explicitly? (default to on if input is shown)
  ```
  define waveFunc(prefactor, epref, axis) {
      directed_exponent = exp![epref, axis] exp![epref, -a0]
      prefactor directed_exponent
  }

  photon = waveFunc![{ 01 31 }, a012, a3]
  ```
  - This will need the LISPy nested environments set-up in order to support nested calls
  - Probably best to do this as a macro system as the evaluator folds the syntax tree eagerly
