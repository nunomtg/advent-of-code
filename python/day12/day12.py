from collections import deque
import time


def height(x: str) -> int:
    if x == "E":
        return 25
    elif x == "S":
        return 0
    return ord(x) - ord("a")


start_time = time.perf_counter_ns()

######################################
input = open("input.txt", "r").read().split("\n")
MAX_ROW, MAX_COL = len(input), len(input[0])
M = [[0 for _ in range(MAX_COL)] for _ in range(MAX_ROW)]
for i in range(MAX_ROW):
    for j in range(MAX_COL):
        if input[i][j] == "S":
            start = (i, j)
        if input[i][j] == "E":
            end = (i, j)
        M[i][j] = height(input[i][j])


def add_borders(row, col, counter):
    borders = []
    for drow, dcol in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        nrow, ncol = row + drow, col + dcol
        if (
            0 <= nrow < MAX_ROW
            and 0 <= ncol < MAX_COL
            and M[nrow][ncol] - M[row][col] <= 1
        ):
            borders.append((nrow, ncol, counter + 1))
    return borders


q = deque()
visited = set()
q.append((*start, 0))
while q:
    row, col, c = q.popleft()
    if (row, col) in visited:
        continue
    visited.add((row, col))
    if (row, col) == end:
        print("Part 1: ", c)
        break
    for nrow, ncol, nc in add_borders(row, col, c):
        q.append((nrow, ncol, nc))


input = open("input.txt", "r").read().split("\n")
MAX_ROW, MAX_COL = len(input), len(input[0])
M = [[-M[i][j] for j in range(MAX_COL)] for i in range(MAX_ROW)]

q = deque()
visited = set()
q.append((*end, 0))
while q:
    row, col, c = q.popleft()
    if (row, col) in visited:
        continue
    visited.add((row, col))
    if M[row][col] == 0:
        print("Part 2: ", c)
        break
    for nrow, ncol, nc in add_borders(row, col, c):
        q.append((nrow, ncol, nc))
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
