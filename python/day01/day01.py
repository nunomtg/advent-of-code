import time

start_time = time.perf_counter_ns()

######################################
elfs = []
with open("input.txt", "r") as f:
    curr = 0
    for line in f:
        line = line.strip()
        if line == "":
            elfs.append(curr)
            curr = 0
            continue
        curr += int(line)


elfs = sorted(elfs)

p1 = elfs[-1]
p2 = sum(elfs[-3:])

print(f"Part 1: {p1}")
print(f"Part 2: {p2}")
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-3:.3f} us")
