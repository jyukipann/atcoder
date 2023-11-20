import numpy as np
N, M = list(map(int, input().split()))
A = np.array(list(map(int, input().split())), dtype=int) - 1
B = np.array(list(map(int, input().split())), dtype=int) - 1
X = np.zeros((N, ), dtype=int) - 1
graph = {i:list() for i in range(N)}
for i in range(M):
    graph[A[i]].append(B[i]) 
    graph[B[i]].append(A[i])

stack = graph[i]
while len(stack) > 0:
    node = stack.pop(0)
    if X[i] == -1:
        stack = graph[node] + stack
    else:
        