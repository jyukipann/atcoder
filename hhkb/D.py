import numpy as np
N, M = list(map(int, input().split()))
A = np.array(list(map(int, input().split())), dtype=int)-1
B = np.array(list(map(int, input().split())), dtype=int)-1
X = np.zeros((N, ), dtype=int)



already_invert = set()
for i in range(M):
    # A[i] == B[i] のとき絶対に無理
    if A[i] == B[i]:
        print('No')
        exit(0)
    
    # 変更する
    if X[A[i]] == X[B[i]]:
        # A[i]を優先
        if A[i] in already_invert:
                print('No')
                exit(0)
        X[A[i]] ^= 1
        already_invert.add(A[i])
print('Yes')

# 出力
# XA = X[A]
# XB = X[B]
# print(XA, XB)