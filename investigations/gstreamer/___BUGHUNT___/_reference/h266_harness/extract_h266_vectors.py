import re, sys

src = open(sys.argv[1]).read()
outdir = sys.argv[2]

# find `type name[] = { ... };` blocks with hex bytes
pattern = re.compile(r'(?:static const guint8|unsigned char)\s+(\w+)\[\]\s*=\s*\{([^}]*)\};', re.DOTALL)
count = 0
for m in pattern.finditer(src):
    name = m.group(1)
    body = m.group(2)
    nums = re.findall(r'0x[0-9a-fA-F]{2}', body)
    if not nums:
        continue
    data = bytes(int(n, 16) for n in nums)
    with open(f"{outdir}/{name}.bin", "wb") as fh:
        fh.write(data)
    count += 1
print(f"extracted {count} vectors")
