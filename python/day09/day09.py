import time


class Node:
    def __init__(self) -> None:
        self.row = 0
        self.col = 0

    def __repr__(self) -> str:
        return f"({self.row}, {self.col})"


def update_tail(tail: Node, head: Node):
    delta_row = head.row - tail.row
    delta_col = head.col - tail.col
    match (delta_row, delta_col):
        case (0, 2):
            tail.col += 1
        case (0, -2):
            tail.col -= 1
        case (2, 0):
            tail.row += 1
        case (-2, 0):
            tail.row -= 1
        case (1, 2):
            tail.row += 1
            tail.col += 1
        case (1, -2):
            tail.row += 1
            tail.col -= 1
        case (-1, 2):
            tail.row -= 1
            tail.col += 1
        case (-1, -2):
            tail.row -= 1
            tail.col -= 1
        case (2, 1):
            tail.row += 1
            tail.col += 1
        case (2, -1):
            tail.row += 1
            tail.col -= 1
        case (-2, 1):
            tail.row -= 1
            tail.col += 1
        case (-2, -1):
            tail.row -= 1
            tail.col -= 1
        case (2, 2):
            tail.row += 1
            tail.col += 1
        case (2, -2):
            tail.row += 1
            tail.col -= 1
        case (-2, 2):
            tail.row -= 1
            tail.col += 1
        case (-2, -2):
            tail.row -= 1
            tail.col -= 1
        case _:
            pass


start_time = time.perf_counter_ns()


######################################

visited_1, visited_2 = set(), set()
knots = [Node() for _ in range(10)]
visited_1.add((0, 0))
visited_2.add((0, 0))
with open("input.txt", "r") as f:
    for line in f:
        a, b = line.split()
        dir, steps = a, int(b)
        match dir:
            case "U":
                for _ in range(steps):
                    knots[0].row += 1
                    for i, (tail, head) in enumerate(zip(knots[1:], knots)):
                        update_tail(tail, head)
                    visited_1.add((knots[1].row, knots[1].col))
                    visited_2.add((tail.row, tail.col))
            case "D":
                for _ in range(steps):
                    knots[0].row -= 1
                    for i, (tail, head) in enumerate(zip(knots[1:], knots)):
                        update_tail(tail, head)
                    visited_1.add((knots[1].row, knots[1].col))
                    visited_2.add((tail.row, tail.col))
            case "L":
                for _ in range(steps):
                    knots[0].col -= 1
                    for i, (tail, head) in enumerate(zip(knots[1:], knots)):
                        update_tail(tail, head)
                    visited_1.add((knots[1].row, knots[1].col))
                    visited_2.add((tail.row, tail.col))
            case "R":
                for _ in range(steps):
                    knots[0].col += 1
                    for i, (tail, head) in enumerate(zip(knots[1:], knots)):
                        update_tail(tail, head)
                    visited_1.add((knots[1].row, knots[1].col))
                    visited_2.add((tail.row, tail.col))

print(f"Part 1: {len(visited_1)}\nPart 2: {len(visited_2)}")

######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
