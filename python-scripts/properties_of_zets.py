"""
Taking a look at the nature of zet conjugates and properties
"""
from arpy import G, config, zet_A, zet_B, zet_E, zet_T
from arpy.algebra.operations import dual, full, hermitian, rev

ZETS = {"ZB": zet_B, "ZT": zet_T, "ZA": zet_A, "ZE": zet_E}
A = dict(zip(config.allowed, [t.alpha for t in G]))


def for_all_zets(func):
    print(func.__qualname__)
    for z, zet in ZETS.items():
        print(f"  {z}: {func(zet)}")
    print()


def sign_check(mvec):
    signs = [t.sign for t in mvec]
    return "".join(["+" if s == 1 else "-" for s in signs])


def sandwich(alpha, func=lambda z: z):
    def _op(z):
        return full(alpha, full(func(z), alpha))

    return _op


@for_all_zets
def zet_hermitian(z):
    return sign_check(hermitian(z))


@for_all_zets
def zet_dual(z):
    return sign_check(dual(z))


@for_all_zets
def zet_reversed(z):
    return sign_check(rev(z))


@for_all_zets
def hermitian_equal_to_reversed(z):
    return hermitian(z) == rev(z)


@for_all_zets
def a0_Z_a0__parity(z):
    return sign_check(sandwich(A["0"])(z))


@for_all_zets
def a0_revZ_a0__hermitian(z):
    return sign_check(sandwich(A["0"], rev)(z))
