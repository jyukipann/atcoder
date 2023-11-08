import numpy as np

n, k = list(map(int, input().split()))
a = np.array(list(map(int, input().split())))
b = np.array(list(map(int, input().split())))
diff_abs = np.abs(a - b)
diff_sum = diff_abs.sum()

if(k >= diff_sum and k % 2 == diff_sum % 2):
    print("Yes")
else:
    print("No")
# print(diff_sum)