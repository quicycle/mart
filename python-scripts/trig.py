#! /usr/bin/env python3
"""
Generating terms for feeding into wolfram alpha for inverting wavefunctions
"""


def electron():
    """
    psi = H_0 R exp[Rw a012 (z a3 - t a0)]
    """
    p = "cos(x)cos(y)"
    q = "sin(x)cos(y)"
    b = "cos(x)sin(y)"
    e = "sin(x)sin(y)"

    print("ap")
    print(f"({p})^3 + {p}({b})^2 + {p}({q})^2 - {p}({e})^2 + 2{q}{b}{e}")

    print("a12")
    print(f"({p})^2({b}) + 2{p}{q}{e} + ({b})^3 - ({q})^2({b}) + {b}({e})^2")

    print("a0123")
    print(f"-({p})^2({q}) - 2{p}{b}{e} + ({q})({b})^2 - ({q})^3 - {q}({e})^2")

    print("a03")
    print(f"-{e}({p})^2 + 2{p}{q}{b} + ({e})({b})^2 + ({e})({q})^2 + ({e})^3")


def photon():
    """
    psi = H_0 R (a01 + a31) exp[Rw a012 (z a3 - t a0)]
    """
    by = ex = "cos(x)"
    bx = ey = "sin(x)"

    print("a23")
    print(
        f"-({bx})^3 - ({bx})({by})^2 - ({bx})({ex})^2 + ({bx})({ey})^2 + 2({by})({ex})({ey})"
    )

    print("a31")
    print(
        f"-({by})({bx})^2 + 2({bx})({ex})({ey}) - ({by})^3 + ({by})({ex})^2 - ({by})({ey})^2"
    )

    print("a01")
    print(
        f"({ex})({bx})^2 - 2({bx})({by})({ey}) - ({ex})({by})^2 + ({ex})^3 + ({ex})({ey})^2"
    )

    print("a02")
    print(
        f"({ey})({bx})^2 + 2({bx})({by})({ex}) - ({ey})({by})^2 - ({ey})({ex})^2 - ({ey})^3"
    )


photon()

# ζ:: {01 31} {p 0123} {p -12}
# {
#   a23( -ξp.ξ12, +ξp.ξ0123 )  -> -cosAsinB + cosBsinA  =  sin(A-B)
#   a31( +ξp^2, +ξ12.ξ0123 )   ->  cosAcosB + sinAsinB  =  cos(A-B)
#   a01( +ξp^2, +ξ12.ξ0123 )   ->  cosAcosB + sinAsinB  =  cos(A-B)
#   a02( -ξp.ξ0123, +ξp.ξ12 )  -> -cosBsinA + cosAsinB  = -sin(A-B)
# }
