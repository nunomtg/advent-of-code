import time
import re

start_time = time.perf_counter_ns()

######################################
data = open("input.txt").read().split("\n")
regex = r"(\d+)-(\d+),(\d+)-(\d+)"
score_1, score_2 = 0, 0
for line in data:
    l1, r1, l2, r2 = map(int, re.match(regex, line).groups())
    range1 = set(range(l1, r1 + 1))
    range2 = set(range(l2, r2 + 1))
    disjunction_len = len(range1 | range2)
    if disjunction_len == max(len(range1), len(range2)):
        score_1 += 1
    if disjunction_len != len(range1) + len(range2):
        score_2 += 1

print(f"Part 1: {score_1}")
print(f"Part 2: {score_2}")
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
