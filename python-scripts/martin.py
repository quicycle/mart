'''
Trying to run some of the calculations in Martin's "Quantum mechanical
probability current as electromagnetic 4-current from topological EM fields"
paper through arpy
'''
from arpy import ARContext


# Configure the algebra
allowed = 'p 23 31 12 0 023 031 012 123 1 2 3 0123 10 20 30'.split()
ctx = ARContext(allowed, '+---', 'into', print_all=True)

with ctx as ar:
    # Define the differential operator d
    d = ar('< 0 1 2 3 >')

    # Define the "complex" fields α & β
    # NOTE: This is the notation for assigning a specific _value_ to an alpha/e
    alpha = ar('{ p[αs] 0123[αq] }')
    beta = ar('{ p[βs] 0123[βq] }')

    # It doesn't look like symbolic grouping of del notation works with this
    # ...yet!
    d_beta = ar('d beta')
    A = ar('alpha d_beta')

    dA = ar('d A')
