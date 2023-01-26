from arpy import ar, full
from arpy.algebra.data_types import MultiVector


def conjugate_singlet(m, singlet):
    return MultiVector([t if t.alpha._index == singlet else -t for t in m])


def f(m, singlet):
    conj = conjugate_singlet(m, singlet)
    res = full(m, conj)
    res.cancel_terms()
    return res


def g(m, singlet):
    conj = conjugate_singlet(m, singlet)
    res = ar(f"a{singlet} conj")
    res.cancel_terms()
    return res


m = ar("{ p 23 31 12 0 023 031 012 123 1 2 3 0123 01 02 03 }")

print(g(m, "p"))
print(g(m, "0"))
print(g(m, "123"))
print(g(m, "0123"))
