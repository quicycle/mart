"""
Just a daft little script that prints out tikz commands to
draw out subspaces of a four dimensional hypercube.
"""

A, B, C, D = (0, 2), (0, 5), (2, 0), (2, 3)
E, F, G, H = (2, 4), (2, 7), (3, 2), (3, 5)
I, J, K, L = (4, 2), (4, 5), (5, 0), (5, 3)
M, N, O, P = (5, 4), (5, 7), (7, 2), (7, 5)

SIDES = {
    "0": [(A, C), (B, D), (E, I), (F, J), (G, K), (H, L), (M, O), (N, P)],
    "1": [(A, G), (B, H), (C, K), (D, L), (E, M), (F, N), (I, O), (J, P)],
    "2": [(A, B), (C, D), (E, F), (G, H), (I, J), (K, L), (M, N), (O, P)],
    "3": [(A, E), (B, F), (C, I), (D, J), (G, M), (H, N), (K, O), (L, P)],
}


def draw_side(lines, offset=0):
    section = ["\\begin{tikzpicture}"]
    for start, end in lines:
        start = tuple(map(lambda n: n + offset, start))
        end = tuple(map(lambda n: n + offset, end))
        section.append(f"    \\draw {start} -- {end};")

    section.append("\\end{tikzpicture}")

    return "\n".join(section)


print(draw_side(SIDES["1"] + SIDES["2"] + SIDES["3"]))
print(draw_side(SIDES["0"] + SIDES["1"] + SIDES["2"] + SIDES["3"]))
