PY_CODEC = {
    "WINDOWS_31J": "shift_jis",
    "WINDOWS_949": "cp949",
}

mismatches = []
both_success_mismatches = []
total = 0
checked = 0

with open("charset_dump.tsv", encoding="utf-8") as f:
    for line in f:
        parts = line.rstrip("\n").split("\t")
        name, hexbytes = parts[0], parts[1]
        rust_cps = parts[2] if len(parts) > 2 else ""
        raw = bytes.fromhex(hexbytes)
        if name in PY_CODEC:
            if raw[0] < 0x81:
                continue  # skip ASCII-lead noise, only genuine double-byte lead range
            codec = PY_CODEC[name]
            try:
                py_s = raw.decode(codec)
                py_cps = ",".join(f"{ord(c):04X}" for c in py_s)
            except UnicodeDecodeError:
                py_cps = "<ERR>"
        elif name == "ISO_2022_JP_0208":
            full = b"\x1b$B" + raw
            try:
                py_s = full.decode("iso2022_jp")
                py_cps = ",".join(f"{ord(c):04X}" for c in py_s)
            except UnicodeDecodeError:
                py_cps = "<ERR>"
        else:
            continue
        total += 1
        checked += 1
        if py_cps != rust_cps:
            mismatches.append((name, hexbytes, rust_cps, py_cps))
            if py_cps != "<ERR>" and rust_cps != "<ERR>":
                both_success_mismatches.append((name, hexbytes, rust_cps, py_cps))

print(f"genuine double-byte rows checked: {total}")
print(f"mismatches (any kind): {len(mismatches)}")
print(f"mismatches where BOTH decoded successfully but to a DIFFERENT codepoint (the interesting class): {len(both_success_mismatches)}")
for m in both_success_mismatches[:80]:
    print(m)
