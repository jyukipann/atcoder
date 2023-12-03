import numpy as np
n = int(input())
a = list(map(int, input().split()))
ia_list = list(enumerate(a))
ia_list.sort(key=lambda x: x[1])
ia_list = np.array(ia_list)

b = [0] * n
for i, av in ia_list:
    ia_list = ia_list[ia_list[:, 1] > av, :]
    b[i] = ia_list[:, 1].sum()
print(' '.join(b))
