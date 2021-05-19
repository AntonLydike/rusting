#!/usr/bin/env python3

from math import ceil

data = [
 (0,   7, 100),
 (32, 107, 203),
 (237, 255, 255),
 (255, 170,   0),
 (0,   2,   0),
 (0,0,0)
]


STEPS = 255

lens = [ 0,
ceil(255 * 0.16),
ceil(255 * 0.42),
ceil(255 * 0.6425),
ceil(255 * 0.8575),
STEPS]



ls = "["

for start, stop, c1, c2 in zip(lens[:-1], lens[1:], data[:-1], data[1:]):
    s = stop - start
    for i in range(s):
        m1, m2 = 1-i/s, i/s
        ls+=f"vec![{ceil(c1[0] * m1 + c2[0] * m2)},{ceil(c1[1] * m1 + c2[1] * m2)},{ceil(c1[2] * m1 + c2[2] * m2)}], "

print(ls + ']')




