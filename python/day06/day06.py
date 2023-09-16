import time

start_time = time.perf_counter_ns()

######################################
data = open("input.txt").read().split("\n")[0]

part_1 = 4
part_2 = 14
for l in zip(data, data[1:], data[2:], data[3:]):
    if len(set(l)) == 4:
        break
    part_1 += 1

for l in zip(
    data,
    data[1:],
    data[2:],
    data[3:],
    data[4:],
    data[5:],
    data[6:],
    data[7:],
    data[8:],
    data[9:],
    data[10:],
    data[11:],
    data[12:],
    data[13:],
):
    if len(set(l)) == 14:
        break
    part_2 += 1

print(f"Part 1: {part_1}")
print(f"Part 1: {part_2}")
# The above is actually more performant than this:
"""
for i in range(len(data)):
    if len(set(data[i:i+4])) == 4:
        part_1 = i+4
        break
    

for i in range(len(data)):
    if len(set(data[i:i+14])) == 14:
        part_2 = i+14
        break
"""

######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
