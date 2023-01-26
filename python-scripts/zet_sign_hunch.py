'''
Checking a hunch about Zet sign behaviour.

It is possible to express all 16 elements of the Algebra via the following
construction based on ζB = {ap, a23, a31, a12}:
    ζT = a0 ^ ζB
    ζE = ζB! ^ a0123
    ζA = a0 ^ ζB! ^ a0123

NOTE: `!` is used here to represent the Hermitian conjugate of the MultiVector
      where all components that square to -ap have their signs inverted.

Using this, it should be possible to investigate the signs of terms as a
result of their constituant parts and see if there is more of a pattern
to the sign distribution.


$ Run this file with `python3.6 -m pytest zet_sign_hunch.py -v`
'''
import sys
import pytest
from arpy import zet_B, ARContext, config


def _block_func(s):
    def func(i, j, cfg=config):
        with ARContext(cfg=cfg) as ar:
            return ar(s).sign

    return func


# Given the construction outlined above, we can define the product of all
# 16 of the `ζ ^ ζ` blocks in terms of their decomposed structure as follows.
BLOCKS = {
    # B row
    'BB': _block_func('i ^ j'),
    'BT': _block_func('i ^ a0 ^ j'),
    'BA': _block_func('i ^ a0 ^ (j!) ^ a0123'),
    'BE': _block_func('i ^ (j!) ^ a0123'),
    # T row
    'TB': _block_func('a0 ^ i ^ j'),
    'TT': _block_func('a0 ^ i ^ a0 ^ j'),
    'TA': _block_func('a0 ^ i ^ a0 ^ (j!) ^ a0123'),
    'TE': _block_func('a0 ^ i ^ (j!) ^ a0123'),
    # A row
    'AB': _block_func('a0 ^ (i!) ^ a0123 ^ j'),
    'AT': _block_func('a0 ^ (i!) ^ a0123 ^ a0 ^ j'),
    'AA': _block_func('a0 ^ (i!) ^ a0123 ^ a0 ^ (j!) ^ a0123'),
    'AE': _block_func('a0 ^ (i!) ^ a0123 ^ (j!) ^ a0123'),
    # E row
    'EB': _block_func('(i!) ^ a0123 ^ j'),
    'ET': _block_func('(i!) ^ a0123 ^ a0 ^ j'),
    'EA': _block_func('(i!) ^ a0123 ^ a0 ^ (j!) ^ a0123'),
    'EE': _block_func('(i!) ^ a0123 ^ (j!) ^ a0123'),
}


def get_signs(block):
    '''Find the ordered signs of a given block'''
    signs = []
    for i in zet_B:
        i = i.extract_alpha()
        for j in zet_B:
            j = j.extract_alpha()
            signs.append(BLOCKS[block](i, j, config))

    return tuple(signs)


# The following sets of components should be equivalent at the sign level
A = {'BB', 'BT', 'TB', 'TT'}
B = {'BA', 'BE', 'TA', 'TE'}
C = {'AB', 'EB'}
C_bar = {'AT', 'ET'}
D = {'AA', 'EA'}
D_bar = {'AE', 'EE'}


res_A = set(map(get_signs, A))
res_B = set(map(get_signs, B))
res_C = set(map(get_signs, C))
res_D = set(map(get_signs, D))
res_C_bar = set(map(get_signs, C_bar))
res_D_bar = set(map(get_signs, D_bar))


def test_blocks_are_equal():
    '''Each of the blocks have consistant sign distributions.'''
    assert len(res_A) == 1
    assert len(res_B) == 1
    assert len(res_C) == 1
    assert len(res_D) == 1
    assert len(res_C_bar) == 1
    assert len(res_D_bar) == 1


def test_inversions_are_correct():
    '''The inverted blocks have consistant sign distributions.'''
    assert list(res_C)[0] == tuple(c * -1 for c in list(res_C_bar)[0])
    assert list(res_D)[0] == tuple(c * -1 for c in list(res_D_bar)[0])


if __name__ == '__main__':
    try:
        test_blocks_are_equal()
        print(test_blocks_are_equal.__doc__)
    except AssertionError:
        print("Blocks are not consistant")
        sys.exit(-1)

    try:
        test_inversions_are_correct()
        print(test_inversions_are_correct.__doc__)
    except AssertionError:
        print("Inversions are not correct")
        sys.exit(-1)
