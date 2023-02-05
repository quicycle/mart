"""
This hard codes the 16 Maxwell compatible cases found using ./check_consistency_with_maxwell.py
and computes the full derivative under each algebra (DG G) and the full self product (G G).

In order to quickly identify differences in the signs of terms (as the _forms_ will all be matching)
I am treating the sign of each of the 256 terms as a bit in a 256-bit number and then printing that
resulting number in hex.
"""

from collections import namedtuple
from arpy import ARContext

Candidate = namedtuple("candidate", "allowed div metric negated")


def allowed_repr(allowed):
    def as_group(ix):
        remap = {"0": "0", "1": "i", "2": "j", "3": "k"}
        return "".join(remap[c] for c in allowed[ix])

    B = as_group(1)
    T = as_group(5)
    E = as_group(13)
    h = as_group(8)
    q = as_group(12)

    return f"p 0 {h} {q} | {B} {T} i {E}"


def all_candidates():
    return [
        # Case I
        Candidate(
            allowed="p 32 13 21 0 032 013 021 321 1 2 3 0123 10 20 30".split(),
            div="by",
            metric="+---",
            negated=False
        ),
        Candidate(
            allowed="p 23 31 12 0 023 031 012 123 1 2 3 0123 01 02 03".split(),
            div="into",
            metric="+---",
            negated=False
        ),
        Candidate(
            allowed="p 32 13 21 0 032 013 021 321 1 2 3 0123 10 20 30".split(),
            div="by",
            metric="-+++",
            negated=True
        ),
        Candidate(
            allowed="p 23 31 12 0 023 031 012 123 1 2 3 0123 01 02 03".split(),
            div="into",
            metric="-+++",
            negated=True
        ),
        # Case II
        Candidate(
            allowed="p 32 13 21 0 032 013 021 321 1 2 3 1230 10 20 30".split(),
            div="by",
            metric="+---",
            negated=False
        ),
        Candidate(
            allowed="p 23 31 12 0 023 031 012 123 1 2 3 1230 01 02 03".split(),
            div="into",
            metric="+---",
            negated=False
        ),
        Candidate(
            allowed="p 32 13 21 0 032 013 021 321 1 2 3 1230 10 20 30".split(),
            div="by",
            metric="-+++",
            negated=True
        ),
        Candidate(
            allowed="p 23 31 12 0 023 031 012 123 1 2 3 1230 01 02 03".split(),
            div="into",
            metric="-+++",
            negated=True
        ),
        # Case III
        Candidate(
            allowed="p 32 13 21 0 023 031 012 123 1 2 3 0123 10 20 30".split(),
            div="by",
            metric="-+++",
            negated=False
        ),
        Candidate(
            allowed="p 23 31 12 0 032 013 021 321 1 2 3 0123 01 02 03".split(),
            div="into",
            metric="-+++",
            negated=False
        ),
        Candidate(
            allowed="p 32 13 21 0 023 031 012 123 1 2 3 0123 10 20 30".split(),
            div="by",
            metric="+---",
            negated=True
        ),
        Candidate(
            allowed="p 23 31 12 0 032 013 021 321 1 2 3 0123 01 02 03".split(),
            div="into",
            metric="+---",
            negated=True
        ),
        # Case IV
        Candidate(
            allowed="p 32 13 21 0 023 031 012 123 1 2 3 1230 10 20 30".split(),
            div="by",
            metric="-+++",
            negated=False
        ),
        Candidate(
            allowed="p 23 31 12 0 032 013 021 321 1 2 3 1230 01 02 03".split(),
            div="into",
            metric="-+++",
            negated=False
        ),
        Candidate(
            allowed="p 32 13 21 0 023 031 012 123 1 2 3 1230 10 20 30".split(),
            div="by",
            metric="+---",
            negated=True
        ),
        Candidate(
            allowed="p 23 31 12 0 032 013 021 321 1 2 3 1230 01 02 03".split(),
            div="into",
            metric="+---",
            negated=True
        ),
    ]


def bit_sign(m, negated):
    # flipping this based on negated maxwell helps show that its not EVERYTHING that negates
    # on = -1 if negated else 1
    on = 1
    k = 0
    
    for (n, term) in enumerate(m):
        if term.sign == on:
            k += 2**n

    return hex(k)

def summarise(c):
    context = ARContext(allowed=c.allowed, div=c.div, metric=c.metric)
    
    print(f"{c.metric} {c.div.ljust(4)} {allowed_repr(c.allowed)} -> ", end="", flush=True)

    with context as ar:
        m = ar("{%s}" % " ".join(ar.allowed))
        d = ar("<%s>" % " ".join(ar.allowed))
        
        DGG = ar("d m")
        print(bit_sign(DGG, c.negated), end=" ", flush=True)

        GG = ar("m m")
        print(bit_sign(GG, c.negated))

if __name__ == "__main__":
    for candidate in all_candidates():
        summarise(candidate)
