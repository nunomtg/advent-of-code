import time

start_time = time.perf_counter_ns()

######################################
# X, Y, Z
draw = ["X", "Y", "Z"]
win = ["Y", "Z", "X"]
lose = ["Z", "X", "Y"]

score_1, score_2 = 0, 0
with open("input.txt", "r") as f:
    data = f.read().splitlines()
    for line in data:
        p1, p2 = line.split()
        idx = "ABC".index(p1)
        if p2 == win[idx]:
            score_1 += 6 + ord(p2) - ord("X") + 1
        elif p2 == draw[idx]:
            score_1 += 3 + ord(p2) - ord("X") + 1
        elif p2 == lose[idx]:
            score_1 += ord(p2) - ord("X") + 1

        if p2 == "X":
            score_2 += ord(lose[idx]) - ord("X") + 1
        elif p2 == "Y":
            score_2 += 3 + ord(draw[idx]) - ord("X") + 1
        elif p2 == "Z":
            score_2 += 6 + ord(win[idx]) - ord("X") + 1

print(f"Part 1: {score_1}")
print(f"Part 2: {score_2}")
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
