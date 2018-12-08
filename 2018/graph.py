#!/usr/bin/env python3

# sudo apt install graphviz
# graph.py input/07_input.txt > dot.dot
# dot -Tpng dot.dot  > dot.png

import re
import sys

pattern = re.compile('Step (.) must be finished before step (.) can begin')

print('digraph graphname {')
for line in open(sys.argv[1]):
    a, b = (pattern.match(line).groups())
    print('{} -> {};'.format(a, b)) 
print('}')
