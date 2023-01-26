"""
Looking at AR-like systems without sign
"""
# NOTE: Color codes can be obtained / checked from running the following:
# $ for i in {0..255} ; do
#     printf "\x1b[48;5;%sm%3d\e[0m " "$i" "$i"
#     if (( i == 15 )) || (( i > 15 )) && (( (i-15) % 6 == 0 )); then
#         printf "\n";
#     fi
# done

from typing import Set
from string import printable
from functools import reduce
from itertools import combinations
from argparse import ArgumentParser


# 22, 88, 124, 160
COLOR_CODE_OFFSET = 124


def compose(a: Set, b: Set) -> str:
    s = a.symmetric_difference(b)
    return "".join(sorted(s))


def colored_by_index(s):
    i = printable.index(s) + COLOR_CODE_OFFSET
    return f"\x1b[48;5;{str(i).rjust(3, '0')}m{s}"


def flatten(col):
    if isinstance(col, list):
        return reduce(lambda a, b: a + b, map(flatten, col))
    else:
        return [col]


class System:
    def __init__(self, order=4):
        self._order = order
        self._indices = [str(n) for n in range(self._order)]
        self._elements = flatten(
            [[set(c) for c in combinations(self._indices, k)] for k in range(self._order + 1)]
        )
        self._labels = dict(zip(["".join(sorted(e)) for e in self._elements], printable))

    def cayley(self, raw=True):
        for a in self._elements:
            row = []
            for b in self._elements:
                product = compose(a, b)

                if raw:
                    product = "p" if product == "" else product
                    row.append(product.ljust(self._order))
                else:
                    row.append(colored_by_index(self._labels[product]))

            print(" ".join(row) + " \x1b[0m")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("--order", type=int, default=4)
    parser.add_argument("--colored", action="store_true")
    args = parser.parse_args()

    s = System(args.order)
    s.cayley(raw=args.colored is False)
