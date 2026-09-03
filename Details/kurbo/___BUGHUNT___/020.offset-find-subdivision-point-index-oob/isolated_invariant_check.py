import math, struct, itertools

def next_up(x):
    if x == 0.0:
        return 5e-324
    b = struct.pack('<d', x)
    i = struct.unpack('<q', b)[0]
    if x > 0:
        i += 1
    else:
        i -= 1
    return struct.unpack('<d', struct.pack('<q', i))[0]

def next_down(x):
    return -next_up(-x)

def test(a):
    n = len(a)
    s = 0.0
    for v in a:
        s += abs(v)
    target = s * 0.5
    i = 0
    steps = 0
    while True:
        if i >= n:
            return ("OOB", i, steps, s, target)
        if abs(a[i]) < target:
            target -= abs(a[i])
            i += 1
            steps += 1
        else:
            break
    return ("ok", i, steps, s, target)

pi = math.pi
interesting = set()
base = [0.0, 5e-324, 1e-300, 1e-16, 1e-10, pi/4, pi/2, pi*0.75, pi, pi-1e-16, pi-1e-10]
for b in base:
    interesting.add(b)
    interesting.add(next_up(b))
    interesting.add(next_down(b))
interesting = sorted(interesting)
print("grid size", len(interesting))

found = 0
for n in (2,3):
    for combo in itertools.product(interesting, repeat=n):
        res = test(list(combo))
        if res[0] == "OOB":
            found += 1
            if found <= 10:
                print(n, combo, res)
print("total OOB found:", found, "grid combos tested:", len(interesting)**2 + len(interesting)**3)
