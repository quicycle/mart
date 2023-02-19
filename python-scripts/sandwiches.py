#! /usr/bin/env python3
"""
Compute the double-sided diamond sandwich product of each unit form by a given
alpha. In the case of a bivector this constitutes a rotation for jk and a
Lorentz boost for 0i.

The results are given in simplified form following application of the following
trig identites:

sin^2(k) + cos^2(k) = 1
sin(2k) = 2.sin(k).cos(k)
cos(2k) = cos^2(k) - sin^2(k)

cosh^2(k) - sinh^2(k) = 1
sinh(2k) = 2.sinh(k).cosh(k)
cosh(2k) = sinh^2(k) + cosh^2(k)


To compute in full with arpy to verify this, you can do the following:

def sandwich(a: str, m: MultiVector, b: str) -> MultiVector:
    res = reduce(full, [MultiVector(a), m, MultiVector(b)])
    return res.cancel_terms()


>>> sandwich('p[cos(k)] 23[sin(k)]', G, 'p[cos(k)] -23[sin(k)]')
>>> sandwich('p[cosh(k)] 01[sinh(k)]', G, 'p[cosh(k)] -01[sinh(k)]')
"""
from argparse import ArgumentParser, RawDescriptionHelpFormatter
from collections import defaultdict
from functools import reduce
from typing import Optional

from arpy import Alpha, MultiVector, config, full, inverse


def squash(s: str, e: str) -> bool:
    a, b = Alpha(s), Alpha(e)
    res = reduce(full, [a, b, a])
    is_positive = res._sign == 1

    return is_positive if inverse(a) == -a else not is_positive


def commutator(a: str, e: str) -> Optional[str]:
    a, b = MultiVector(a), MultiVector(e)
    res = full(a, b) - full(b, a)
    res.cancel_terms()

    if len(res._terms) == 0:
        return None

    return res._terms[0]


def per_alpha_expressions(s: str):
    a = Alpha(s)
    h = "h" if inverse(a).sign == 1 else ""

    d = defaultdict(list)
    for a in config.allowed:
        d[a].append(f"cos{h}(2θ).ξ{a}" if squash(s, a) else f"ξ{a}")
        t = commutator(s, a)
        if t is not None:
            d[t.index].append(f"{'+' if t.sign == 1 else '-'} sin{h}(2θ).ξ{a}")

    for a in config.allowed:
        terms = d[a]
        s = " ".join(sorted(terms, reverse=True))
        print(f"a{a.ljust(5)} {s}")


if __name__ == "__main__":
    parser = ArgumentParser(description=__doc__, formatter_class=RawDescriptionHelpFormatter)
    parser.add_argument("-a", "--alpha", type=str)
    parser.add_argument("-m", "--metric", type=str, default="+---")
    args = parser.parse_args()

    config.metric = args.metric

    s = f"Result for ψ = X.G.X^<>,   X = e^k.a{args.alpha} = A.ap + B.a{args.alpha}"
    print(f"{s}\n{'=' * len(s)}\n")
    per_alpha_expressions(args.alpha)
