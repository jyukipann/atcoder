import numpy as np

A = np.array([[1,2],[2,4]], dtype=int)
# t[2,1] s[1, 2]
t = np.array([-2,1])
s = np.array([1,2])
tp = A.dot(2*t.T)
sp = A.dot(2*s.T)
tk = tp[0]/t[0] if tp[0]/t[0] == tp[1]/t[1] else "error"
sk = sp[0]/s[0] if sp[0]/s[0] == sp[1]/s[1] else "error"
print(tp,tk)
print(sp,sk)

"""
A
1 2
2 4

t
-2k
1k

At
-2k+2k
-4k+4k

At
0
0

At = 0t

s
1k
2k

As
5k
10k

As = 5s
"""