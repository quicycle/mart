"""
Finding inverses for 'Time + 3xSpace' multivectors
"""
from itertools import product
from arpy import B, T, A, E, MultiVector, Alpha
from arpy.algebra.operations import full, hermitian, diamond, project

p = MultiVector("p")
t = MultiVector("0")
h = MultiVector("123")
q = MultiVector("0123")

four_things = product([p, t, h, q], [B, T, A, E])
alpha_p = Alpha("p")


def simple_rep(m):
    alphas = set(t._alpha for t in m._terms)
    return sorted(list(alphas))


def scalar_value(m):
    return str(m)[11 : str(m).index(")") - 1]


def double_dagger(m):
    fields = project(m, 2)
    return (fields + fields - m).cancel_terms()


def squared(m):
    """psi squared"""
    return full(m, m).cancel_terms()


def psi_psi_dagger(m):
    """psi psi^{!}"""
    return full(m, hermitian(m)).cancel_terms()


def psi_psi_ddagger(m):
    """psi psi^{!!}"""
    return full(m, double_dagger(m)).cancel_terms()


def vdm_scalar(m):
    """vdM scalar"""
    phi = full(m, hermitian(m)).cancel_terms()
    return full(phi, diamond(phi)).cancel_terms()


for time_like, space_like in four_things:
    mvec = time_like + space_like

    # check simplest to most complex
    for f in [squared, psi_psi_dagger, psi_psi_ddagger, vdm_scalar]:
        res = f(mvec)
        alphas = simple_rep(res)
        if len(alphas) == 1 and alphas[0] == alpha_p:
            print(f"{f.__doc__}: {simple_rep(mvec)}\n  {scalar_value(res)}")
            print()
            break
