#!/usr/bin/env arc

#SIMPLIFY_MULTIVECTORS
#SHOW_COMMENTS
#SHOW_INPUT

/*
Looking at generalised differentials based on the Zets.
*/

// Pure Zet differentials
dB = <p 23 31 12>
dT = <0 023 031 012>
dA = <123 1 2 3>
dE = <0123 01 02 03>

// Applied to zB
dB zB
dT zB
dA zB
dE zB


// Applied to same zet
dB zB
dT zT
dA zA
dE zE

// Applied to dual zet
dB zE
dT zA
dA zT
dE zB
