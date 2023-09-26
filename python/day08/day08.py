import numpy as np
import time

start_time = time.perf_counter_ns()

######################################
data = open("input.txt").read().rstrip().split("\n")
M = np.array([[int(c) for c in line] for line in data], dtype=int)
ROW, COL = M.shape

part_1 = 2 * COL + 2 * ROW - 4
part_2 = 0
for row in range(1, ROW - 1):
    for col in range(1, COL - 1):
        # up = np.flip(M[:row, col])
        up = M[:row, col][::-1]
        down = M[row + 1 :, col]
        # left = np.flip(M[row, :col])
        left = M[row, :col][::-1]
        right = M[row, col + 1 :]
        if (
            M[row, col] > np.max(up)
            or M[row, col] > np.max(down)
            or M[row, col] > np.max(left)
            or M[row, col] > np.max(right)
        ):
            part_1 += 1
        score = 1
        for side in [up, down, left, right]:
            counter = 0
            for tree in side:
                counter += 1
                if tree >= M[row, col]:
                    break
            score *= counter
        part_2 = max(part_2, score)


print("Part 1: ", part_1)
print("Part 2: ", part_2)
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
