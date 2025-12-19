rng, ing = open(0).read().split("\n\n")
rng = sorted((*map(int, x.split("-")),) for x in rng.split())
ing = [*map(int, ing.split())]

tem = [[-1, -2]]
for p, q in rng:
    if p - 1 > (n := tem[-1][1]):
        tem.append([p, q])
    tem[-1][1] = max(n, q)

p1 = p2 = 0
for p, q in tem:
    p1 += sum(p <= x <= q for x in ing)
    p2 += q - p + 1

print(f"silver: {p1}, gold: {p2}")