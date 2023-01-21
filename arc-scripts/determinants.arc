#!/usr/bin/env arc

#SIMPLIFY_MULTIVECTORS
#SHOW_COMMENTS
#SHOW_INPUT

/*
Looking at 'Inverse and Determinant in 0 to 5 Dimensional Clifford Algebra'
  https://arxiv.org/pdf/1104.0067.pdf

Equations 10a and 10b give expressions for the determinant of a Multivector in a
four dimensional Clifford Algebra. Computing this expression results in the
negative of the van der Mark scalar and can be obtained in multiple different
ways. Doing so requires computing grade negated conjugates of the input
multivector and the notation in the paper ends up being rather verbose with
brackets...instead, the arc notation show below was previously defined but is
the inverse of what is shown in the paper.

In the paper, [[X]]_ab denotes negating all elements of grade a or b
In arc, X|ab denotes negating every element NOT of grade a or b

Regardless, the results are the same when supplying the dual set of grades or by
negating the resulting product.

Identifying the invariant scalar as the determinant, allows the numerator of the
vdm inverse as the adjugate (or adjoint).
*/


// 10a
inner = G G|23
detG = -(inner inner|14)

// 10b
inner = G G|12
detG = -(inner inner|34)

// van der Mark
phi = G G!
detG = phi phi|0
