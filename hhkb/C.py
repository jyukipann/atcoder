import numpy as np

A = np.zeros((9,9), dtype=int)
for i in range(9):
  A[i] = list(map(int, input().split()))

# print(A)
s = set(list(range(1,10)))
area_index = [
    (0, 3, 0, 3),
    (3, 6, 0, 3),
    (6, 9, 0, 3),
    (0, 3, 3, 6),
    (3, 6, 3, 6),
    (6, 9, 3, 6),
    (0, 3, 6, 9),
    (3, 6, 6, 9),
    (6, 9, 6, 9),
]
# 各行9 各列 9 各エリア 9
for i, a in enumerate(area_index):
    row_s = set(A[i, :].ravel())
    col_s = set(A[:, i].ravel())
    are_s = set(A[a[0]:a[1], a[2]:a[3]].ravel())
    if row_s != s or col_s != s or are_s != s:
        print("No")
        break
else:
    print("Yes")


""" input
1 2 3 4 5 6 7 8 9
4 5 6 7 8 9 1 2 3
7 8 9 1 2 3 4 5 6
2 3 4 5 6 7 8 9 1
5 6 7 8 9 1 2 3 4
8 9 1 2 3 4 5 6 7
3 4 5 6 7 8 9 1 2
6 7 8 9 1 2 3 4 5
9 1 2 3 4 5 6 7 8
"""