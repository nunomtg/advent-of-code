import time


def check(X: int, cycle: int):
    if cycle in [20, 60, 100, 140, 180, 220]:
        return X * cycle
    return 0


def draw(X: int, cycle: int, board: list):
    curr_pointer = (cycle - 1) % 40
    if abs(curr_pointer - X) <= 1:
        board.append("#")
    else:
        board.append(" ")


start_time = time.perf_counter_ns()

######################################
data = open("input.txt").read().split("\n")

X = 1
cycle = 1
score = 0
board = []

for i in range(len(data)):
    match data[i].split(" "):
        case ("addx", value):
            score += check(X, cycle)
            draw(X, cycle, board)
            cycle += 1

            score += check(X, cycle)
            draw(X, cycle, board)
            cycle += 1

            X += int(value)

        case _:
            score += check(X, cycle)
            draw(X, cycle, board)
            cycle += 1

print("Part 1: ", score)
print("Part 2:")
for c, i in enumerate(board):
    if c % 40 == 0 and c != 0:
        print()
    print(i, end="")
print()
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
