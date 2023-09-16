import time

start_time = time.perf_counter_ns()

######################################
data = open("input.txt").read().split("\n")
score_1 = 0
for line in data:
    middle = len(line) // 2
    c = (set(line[:middle]) & set(line[middle:])).pop()
    if c == c.upper():
        score_1 += ord(c) - ord("A") + 27
    else:
        score_1 += ord(c) - ord("a") + 1

score_2 = 0
for l1, l2, l3 in zip(data[::3], data[1::3], data[2::3]):
    c = (set(l1) & set(l2) & set(l3)).pop()
    if c == c.upper():
        score_2 += ord(c) - ord("A") + 27
    else:
        score_2 += ord(c) - ord("a") + 1

print(f"Part 1: {score_1}")
print(f"Part 2: {score_2}")
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
