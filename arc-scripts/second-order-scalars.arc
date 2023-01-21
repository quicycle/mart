#!/usr/bin/env arc

#SIMPLIFY_MULTIVECTORS
#SHOW_COMMENTS
#SHOW_INPUT


/*
A look at the form of second order inverses and the resulting scalars for the
subset of MultiVectors that support them.

To be concrete: second order inverses are possible for MultiVectors where
there is a single source of triadic elements and there is _either_ a single
zet paired singlet _or_ any number of non-paired singlets.

When this is the case, a second order inverse can be computed using the
following formula for all triads other than B:

    ψ^-1 = ψψ|(p+epsilon) = ψψ^diamond

For B, this is what is required:

    ψ^-1 = ψ(ψ - 2B)


For Zets the invariant scalar is simply ψψ^dagger like so:
*/


zetB = zB zB!
zetT = zT zT!
zetA = zA zA!
zetE = zE zE!

singlets = { p 0 123 0123 }

B2 = B + singlets - { p }
T2 = T + singlets - { 0 }
A2 = A + singlets - { 123 }
E2 = E + singlets - { 0123 }

B2 (B2 - (2 B))
T2 T2|0
A2 A2|0
E2 E2|0

singlets singlets|0


/*
The following are (as yet) uncategorised MultiVectors with second order
inverses. I have a hunch but still need to confirm.
*/

// 2: Different Zets + Different Orientation
{ 01 31 } { 01 31 }|0

// 3: Different Zets + Different Orientation
{ 01 31 3 } { 01 31 3 }!
