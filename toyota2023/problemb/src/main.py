n, d = list(map(int, input().split()))
s = int('0b'+'1'*d, 0)

for i in range(n):
    s_i = int('0b'+input().replace('x', '0').replace('o', '1'), 0)
    s &= s_i

bin_s = bin(s)[2:].split('0')
counts = list(map(len, bin_s))
print(max(counts))







