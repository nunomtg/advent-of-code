import time
from collections import deque
from dataclasses import dataclass
from typing import Callable, List
from math import lcm
from copy import deepcopy


@dataclass
class Monkey:
    items: deque
    inspect_func: Callable
    test_func: Callable
    inspect_counter: int = 0

    def inspect(self, item) -> None:
        self.inspect_counter += 1
        return self.inspect_func(item)

    def test(self, item) -> bool:
        return self.test_func(item)


def isDivisible(n: int, m: int):
    return n % m == 0


def throw(m1: int, m2: int, b: bool):
    return m1 if b else m2


def get_result(M: List[Monkey]) -> int:
    inspect_counter = [m.inspect_counter for m in M]
    inspect_counter.sort()
    return inspect_counter[-2] * inspect_counter[-1]


start_time = time.perf_counter_ns()

######################################
ceiling = lcm(11, 19, 5, 2, 13, 7, 3, 17)

monkey1 = Monkey(
    deque([97, 81, 57, 57, 91, 61]),
    lambda x: x * 7,
    lambda x: throw(5, 6, isDivisible(x, 11)),
)
monkey2 = Monkey(
    deque([88, 62, 68, 90]), lambda x: x * 17, lambda x: throw(4, 2, isDivisible(x, 19))
)
monkey3 = Monkey(
    deque([74, 87]), lambda x: x + 2, lambda x: throw(7, 4, isDivisible(x, 5))
)
monkey4 = Monkey(
    deque([53, 81, 60, 87, 90, 99, 75]),
    lambda x: x + 1,
    lambda x: throw(2, 1, isDivisible(x, 2)),
)
monkey5 = Monkey(
    deque([57]), lambda x: x + 6, lambda x: throw(7, 0, isDivisible(x, 13))
)
monkey6 = Monkey(
    deque([54, 84, 91, 55, 59, 72, 75, 70]),
    lambda x: x * x,
    lambda x: throw(6, 3, isDivisible(x, 7)),
)
monkey7 = Monkey(
    deque([95, 79, 79, 68, 78]),
    lambda x: x + 3,
    lambda x: throw(1, 3, isDivisible(x, 3)),
)
monkey8 = Monkey(
    deque([61, 97, 67]), lambda x: x + 4, lambda x: throw(0, 5, isDivisible(x, 17))
)
M1 = [monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7, monkey8]
M2 = deepcopy(M1)

for i in range(20):
    for m in M1:
        while m.items:
            item = m.items.popleft()
            item = m.inspect(item) // 3
            next_monkey = m.test(item)
            M1[next_monkey].items.append(item)


for i in range(10_000):
    for m in M2:
        while m.items:
            item = m.items.popleft()
            item = m.inspect(item) % ceiling
            next_monkey = m.test(item)
            M2[next_monkey].items.append(item)

print(f"Part 1: {get_result(M1)}")
print(f"Part 2: {get_result(M2)}")
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
