import networkx as nx
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

G = nx.grid_2d_graph(MAX_ROW, MAX_COL, create_using=nx.DiGraph)
G.remove_edges_from(
    [(n1, n2) for n1, n2 in G.edges if M[n2[0]][n2[1]] - M[n1[0]][n1[1]] > 1]
)
solution = nx.shortest_path_length(G, target=end)
print("Part 1: ", solution[start])
print("Part 2: ", min(score for n, score in solution.items() if M[n[0]][n[1]] == 0))
######################################

end_time = time.perf_counter_ns()
execution_time = end_time - start_time

print(f"Python program executed in {execution_time*1e-6:.3f} ms")
