PLT_HDR = 128
PLT_SIZE = 32
NSMALL = (0x100000 - PLT_HDR) // PLT_SIZE


def to_plt_offset(i):
    if i < NSMALL:
        return PLT_HDR + i * PLT_SIZE
    j = i - NSMALL
    return 0x100000 + (j // 160) * 5120 + (j % 160) * 24


def ptr_offset(i, nsyms):
    j = i - NSMALL
    block = j // 160
    nlarge = nsyms - NSMALL
    nstub = min(160, nlarge - block * 160)
    return 0x100000 + block * 5120 + nstub * 24 + (j % 160) * 8


def computed_size(nsyms):
    if nsyms == 0:
        return 0
    return PLT_HDR + nsyms * PLT_SIZE


print("sparc_num_small_plt =", NSMALL)
print("hdr + NSMALL*plt_size =", hex(PLT_HDR + NSMALL * PLT_SIZE))

worst = 0
bad = []
cases = list(range(1, 400)) + [
    NSMALL - 1, NSMALL, NSMALL + 1, NSMALL + 2, NSMALL + 159, NSMALL + 160,
    NSMALL + 161, NSMALL + 319, NSMALL + 320, NSMALL + 321,
    NSMALL + 999, NSMALL + 1000, NSMALL + 5000, NSMALL + 65600, NSMALL + 100000,
]
for n in cases:
    if n <= 0:
        continue
    size = computed_size(n)
    need = PLT_HDR
    for i in range(max(0, n - 1000), n):
        if i < NSMALL:
            need = max(need, to_plt_offset(i) + 32)
        else:
            need = max(need, to_plt_offset(i) + 24)
            need = max(need, ptr_offset(i, n) + 8)
    if need > size:
        bad.append((n, size, need, need - size))
    worst = max(worst, need - size)

print("cases checked:", len(cases))
print("overflowing cases:", bad[:10], "total", len(bad))
print("max (need - size) over all cases:", worst)

for nlarge in (1, 10, 160, 161, 1000, 65600):
    n = NSMALL + nlarge
    size = computed_size(n)
    last = n - 1
    e = max(to_plt_offset(last) + 24, ptr_offset(last, n) + 8)
    print(f"nlarge={nlarge:6d} nsyms={n} sized={size} ({hex(size)}) "
          f"last_end={e} ({hex(e)}) delta={e - size}")
