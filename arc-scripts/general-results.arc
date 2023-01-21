#!/usr/bin/env arc

#SIMPLIFY_MULTIVECTORS
#SHOW_COMMENTS
#SHOW_INPUT


/*
Results for conjugate products of the general multivector
*/

reversed = ({ p 0 -123 0123 } + A - B - T - E)
split = { p 0 123 0123 } - (B + T + A + E)

// Psi squared
G G

// Psi Psi dagger
G G!

// Psi Psi diamond
G G|0

// Psi Psi reversed
G reversed

// Psi diamond Psi reversed
G|0 reversed

// Psi diamond Psi dagger
G|0 G!

G split
