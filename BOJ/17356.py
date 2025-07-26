a, b = map(int, open(0).read().split())
m = (b - a) / 400
res = 1 / (1 + 10**m)
print(res)
