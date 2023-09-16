import time

start_time = time.perf_counter_ns()

######################################
data = open("input.txt").read().split("\n")

# Dictionary that has all dirs and their sizes
dirs = {}
curr_dir = []
for line in data:
    match line.split():
        case ["$", "cd", "/"]:
            curr_dir = ["home"]
            dirs["home"] = 0
        case ["$", "cd", ".."]:
            curr_dir.pop()
        case ["$", "cd", command]:
            curr_dir.append(command)
            dirs["/".join(curr_dir)] = 0
        case ["$", "ls"]:
            continue
        case ["dir", _]:
            continue
        case [size, _]:
            size = int(size)
            dirs["home"] += size
            for i in range(1, len(curr_dir)):
                d = "/".join(curr_dir[: i + 1])
                dirs[d] += size

part_1 = 0
space_needed = 30_000_000 - (70_000_000 - dirs["home"])

# Sort dirs by size
dirs = dict(sorted(dirs.items(), key=lambda item: item[1]))

for dir, size in dirs.items():
    if size <= 100_000:
        part_1 += size

print(f"Part 1: {part_1}")

for dir, size in dirs.items():
    if size >= space_needed:
        print(f"Part 2: {size}")
        break

######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
