#!/usr/bin/env arc

#SIMPLIFY_MULTIVECTORS
#SHOW_COMMENTS
#SHOW_INPUT

/*
Looking at inverses of photon and what I _think_ are electron wave functions

The following wave functions all come from a common basis of johns photon
wave function:
    H_0 R (a01 + a31) exp[Rw a012 (z a3 - t a0)]

The electron wave functions are simply the photon without the external field:
    H_0 R exp[Rw a012 (z a3 - t a0)]

By modifying the signs in the exponent you can produce a family of four wave
functions. The once shown above are referred to as '+-' for the signs of z and
t respectively. In total we have:
  +-  (_pn)
  -+  (_np)
  ++  (_pp)
  --  (_nn)

The forms shown below are after decomposing to sine/cosine and multiplying out
the two sets of brackets (three in the case of photons).
*/

e_pn = { p 0123 -12 03 }
e_np = { p -0123 12 03 }
e_pp = { p 0123 12 -03 }
e_nn = { p -0123 -12 -03 }


e_pn e_np
e_pn e_pn
e_pn e_nn
e_pn e_pp
