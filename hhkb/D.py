import numpy as np
N, M = list(map(int, input().split()))
A = np.array(list(map(int, input().split())), dtype=int)-1
B = np.array(list(map(int, input().split())), dtype=int)-1
X = np.zeros((N, ), dtype=int)
