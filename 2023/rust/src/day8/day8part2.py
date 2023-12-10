'''
2023 Advent of Code - Day 08 (https://adventofcode.com/2023/day/8)
Solution by Sav Bell (https://github.com/savbell)
Challenge: Solve every day in a single line of Python code.
           See full progress at https://github.com/savbell/advent-of-code-one-liners
'''

import itertools as it
import math
import re

# Input files not included in repo per copyright & Eric Wastl's request.
# https://www.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs/
# Replace with path to your personal input file if you would like to run the code.
input_file = '../inputs/day8.txt'


# match the format of input files for the Basilisk.
q = { 8: open(input_file).read().strip() }

######################## PART 2: MULTI-LINE SOLUTION ##########################
instructions = q[8].split('\n')[0]
nodes = [re.findall(r'(\w+) = \((\w+), (\w+)\)', line) for line in q[8].split('\n')[2:]]
first_nodes = [n[0][0] for n in nodes if n[0][0].endswith('A')]
nodes = {n[0][0]: n[0][1:] for n in nodes}

all_steps = []
for node in first_nodes:
    cur = node
    steps = 0
    while not cur.endswith('Z'):
        for d in instructions:
            steps += 1
            if d == 'R':
                cur = nodes[cur][1]
            elif d == 'L':
                cur = nodes[cur][0]
    all_steps.append(steps)

# 13524038372771
print('Day 08 Part 2:',math.lcm(*all_steps))
