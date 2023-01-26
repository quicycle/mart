"""
sign -> oscillation
"""
from arpy import ARContext, commutator, sign_cayley

PRINT_ALL = False
ALLOWED = "p 23 31 12 0 023 031 012 123 1 2 3 0123 01 02 03".split()
I0_ALLOWED = "p 23 31 12 0 023 031 012 123 1 2 3 0123 10 20 30".split()
MAXWELL = [
    [-1, -1, -1],
    [1, -1, 1],
    [-1, 1, 1],
    [1, -1, 1],
    [-1, -1, -1],
    [1, -1, 1],
    [1, 1, -1],
    [1, -1, 1],
]
NEG_MAXWELL = [
    [1, 1, 1],
    [-1, 1, -1],
    [1, -1, -1],
    [-1, 1, -1],
    [1, 1, 1],
    [-1, 1, -1],
    [-1, -1, 1],
    [-1, 1, -1],
]


def compare_configs(allowed=ALLOWED, func=None):
    if func is None:
        return lambda f: compare_configs(allowed=allowed, func=f)

    pmmm = ARContext(allowed, "+---", "into", print_all=PRINT_ALL)
    mppp = ARContext(allowed, "-+++", "by", print_all=PRINT_ALL)

    qname = func.__qualname__
    print(f"\n.: {qname} :.\n{'=' * (len(qname) + 6)}\n{func.__doc__}")

    for ctx in [pmmm, mppp]:
        print("[%s]" % "".join("+" if m == 1 else "-" for m in ctx.metric))
        func(ctx)
        print("")


@compare_configs()
def maxwell_is_maintained(ctx):
    with ctx as ar:
        XiG = "{%s}" % " ".join(ar.allowed)
        Dg = ar("<0 1 2 3> %s" % XiG)

        # Ignoring the sign of pivot terms as they are not governed by maxwell
        maxwell_signs = [
            [
                k.sign
                for k in sorted(Dg[blade], key=lambda k: k._component_partials)
                if k._components[0].val not in ["p", ar.cfg.allowed[-4]]
            ]
            for blade in ["0", "1", "2", "3", "123", "023", "031", "012"]
        ]

        if maxwell_signs == MAXWELL:
            print("Gives conventional Maxwell")
        elif maxwell_signs == NEG_MAXWELL:
            print("Gives inverted Maxwell")
        else:
            print("Does not give Maxwell")

        for i, row in enumerate(Dg.iter_alphas()):
            if i % 4 == 0:
                print()
            print(f"{str(row[0]).ljust(5)} ", end="")
            for t in row[1]:
                sign = "□" if t.sign == 1 else "■"
                print(sign, end=" ")

            print("  " + " ".join(t._repr_no_alpha(count=1)[2:].ljust(7) for t in row[1]))


@compare_configs()
def group_commutator_full(ctx):
    """
    Compute the group commutator [a, b] = (a b a^-1 b^-1) for each combination
    of a and b in the algebra.

    [a, b] =  αp  --> □
    [a, b] = -αp  --> ■
    """
    with ctx as ar:
        sign_cayley(commutator, cfg=ar.cfg)


@compare_configs()
def dgg(ctx):
    """
    Sign of terms when computing DG(G)
    """
    with ctx as ar:
        dgg = ar(f'{"<%s>" % " ".join(ar.allowed)} {"{%s}" % " ".join(ar.allowed)}')

        for i, row in enumerate(dgg.iter_alphas()):
            if i % 4 == 0:
                print()
            print(f"{str(row[0]).ljust(5)} ", end="")

            for j, t in enumerate(row[1]):
                sign = "□" if t.sign == 1 else "■"
                print(sign, end=" ")

                if j % 4 == 3:
                    print(" ", end="")

            print()


@compare_configs()
def gg(ctx):
    """
    Sign of terms when computing G ^ G
    """
    with ctx as ar:
        # gg = ar(f'{"{%s}" % " ".join(ar.allowed)} {"{%s}" % " ".join(ar.allowed)}')
        gg = ar("G ^ G")

        for i, row in enumerate(gg.iter_alphas()):
            if i % 4 == 0:
                print()
            print(f"{str(row[0]).ljust(5)} ", end="")

            for j, t in enumerate(row[1]):
                sign = "□" if t.sign == 1 else "■"
                print(sign, end=" ")

                if j % 4 == 3:
                    print(" ", end="")

            print()
