import re
from copy import deepcopy

import time

start_time = time.perf_counter_ns()

######################################

data = open("input.txt").read().split("\n")

# Parsing this is honestly harder than just writing it by hand.
l1 = list("BGSC")
l2 = list("TMWHJNVG")
l3 = list("MQS")
l4 = list("BSLTWNM")
l5 = list("JZFTVGWP")
l6 = list("CTBGQHS")
l7 = list("TJPBW")
l8 = list("GDCZFTQM")
l9 = list("NSHBPF")

pattern = re.compile(r"move (\d+) from (\d+) to (\d+)")
lists = [[], l1, l2, l3, l4, l5, l6, l7, l8, l9]
lists_2 = deepcopy(lists)
for line in data:
    m = pattern.match(line)
    if m:
        qt, from_, to_ = map(int, m.groups())
        for _ in range(qt):
            lists[to_].append(lists[from_].pop())

        lists_2[to_].extend(lists_2[from_][-qt:])
        lists_2[from_] = lists_2[from_][:-qt]

print("Part 1: ", end="")
for l in lists[1:]:
    print(l[-1], end="")
print("\nPart 2: ", end="")
for l in lists_2[1:]:
    print(l[-1], end="")
print()
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
