#!/usr/bin/env arc

#SHOW_COMMENTS
#SHOW_INPUT

// This is a quick way to try out different prefactors when looking at different wave equations
// of a similar form to John's photon equation.
//
// The third multivector is the fixed exponent obtained via exp[Rw(z.a3 - t.a0)] which will always
// take that form for a z-oriented solution (I _think_ that is correct...)
//

// >>> John's Photon
prefactor = { 01 31 }
epref = a012
exp1 = { p } + (epref { 3 })
exp2 = { p } + (epref { -0 })
exponent = exp1 exp2
waveFunc = prefactor exponent


// >>> My Odd wave
prefactor = { 1 031 }
epref = a012
exp1 = { p } + (epref { 3 })
exp2 = { p } + (epref { -0 })
exponent = exp1 exp2
waveFunc = prefactor exponent
