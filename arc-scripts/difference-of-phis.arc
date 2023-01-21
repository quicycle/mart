#!/usr/bin/env arc

#SIMPLIFY_MULTIVECTORS
#SHOW_COMMENTS
#SHOW_INPUT


/*
Looking into a hunch around N_diamond and something along the lines of
the difference of two squares.
*/

N1 = { 0 123 01 02 03 }
N2 = { 0 123 23 31 12 }
N3 = { p 0123 023 031 012 }
N4 = { p 0123 1 2 3 }


mu1 = ({ p } + N1)({ p } - N1)
mu3 = ({ p } - N3)({ p } - N3!)
mu4 = ({ p } - N4)({ p } - N4!)
