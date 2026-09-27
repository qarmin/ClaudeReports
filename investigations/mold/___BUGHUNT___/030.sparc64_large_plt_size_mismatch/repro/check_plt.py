#!/usr/bin/env python3
"""Validate mold's SPARC64 .plt output against the layout its own code assumes.

Reads a big-endian ELF64 shared object produced by mold and checks, for every
PLT entry:
  * the code stub [to_plt_offset(i), +24 or +32) lies inside .plt's sh_size
  * for large entries, the 8-byte data pointer slot lies inside .plt's sh_size
  * the stub's `ldx [%o7 + imm13], %g1` displacement really points at that slot
  * the matching R_SPARC_JMP_SLOT relocation targets the slot (large) or the
    stub (small)

Usage: check_plt.py <out.so>
"""

import struct
import sys

PLT_HDR, PLT_SIZE = 128, 32
NSMALL = (0x100000 - PLT_HDR) // PLT_SIZE
R_SPARC_JMP_SLOT = 21
BE = ">"


def to_plt_offset(i):
    if i < NSMALL:
        return PLT_HDR + i * PLT_SIZE
    j = i - NSMALL
    return 0x100000 + (j // 160) * 5120 + (j % 160) * 24


def ptr_offset(i, n):
    j = i - NSMALL
    block = j // 160
    nstub = min(160, (n - NSMALL) - block * 160)
    return 0x100000 + block * 5120 + nstub * 24 + (j % 160) * 8


def main():
    data = open(sys.argv[1], "rb").read()
    assert data[:4] == b"\x7fELF" and data[4] == 2 and data[5] == 2, "not BE ELF64"
    shoff, shentsize, shnum, shstrndx = struct.unpack_from(BE + "Q", data, 0x28)[0], \
        struct.unpack_from(BE + "H", data, 0x3a)[0], \
        struct.unpack_from(BE + "H", data, 0x3c)[0], \
        struct.unpack_from(BE + "H", data, 0x3e)[0]

    shdrs = []
    for i in range(shnum):
        o = shoff + i * shentsize
        (name, typ, flags, addr, off, size, link, info, aln,
         ent) = struct.unpack_from(BE + "IIQQQQIIQQ", data, o)
        shdrs.append(dict(name=name, type=typ, flags=flags, addr=addr,
                          off=off, size=size, link=link, info=info,
                          align=aln, entsize=ent))
    strbase = shdrs[shstrndx]["off"]
    for s in shdrs:
        end = data.index(b"\0", strbase + s["name"])
        s["sname"] = data[strbase + s["name"]:end].decode()

    by_name = {s["sname"]: s for s in shdrs}
    plt = by_name[".plt"]
    relplt = by_name[".rela.plt"]
    nrel = relplt["size"] // 24
    print(f".plt      addr=0x{plt['addr']:x} off=0x{plt['off']:x} "
          f"size=0x{plt['size']:x} ({plt['size']}) align={plt['align']}")
    print(f".rela.plt entries={nrel}  (sparc_num_small_plt={NSMALL}, "
          f"large={max(0, nrel - NSMALL)})")

    n = nrel
    expected_size = PLT_HDR + n * PLT_SIZE
    print(f"expected sh_size = 128 + {n}*32 = {expected_size} "
          f"(0x{expected_size:x})  -> {'MATCH' if expected_size == plt['size'] else 'MISMATCH'}")

    errors = []
    high_water = 0
    for i in range(n):
        so = to_plt_offset(i)
        stub_len = PLT_SIZE if i < NSMALL else 24
        high_water = max(high_water, so + stub_len)
        if so + stub_len > plt["size"]:
            errors.append(f"entry {i}: stub [{so},{so + stub_len}) exceeds "
                          f"sh_size {plt['size']}")
        if i >= NSMALL:
            po = ptr_offset(i, n)
            high_water = max(high_water, po + 8)
            if po + 8 > plt["size"]:
                errors.append(f"entry {i}: data ptr [{po},{po + 8}) exceeds "
                              f"sh_size {plt['size']}")
                continue
            # ldx [ %o7 + imm13 ], %g1  is the 4th insn of a large stub
            insn = struct.unpack_from(BE + "I", data, plt["off"] + so + 12)[0]
            imm13 = insn & 0x1fff
            if imm13 & 0x1000:
                imm13 -= 0x2000
            call = plt["addr"] + so + 4
            if call + imm13 != plt["addr"] + po:
                errors.append(
                    f"entry {i}: ldx target 0x{call + imm13:x} != data ptr "
                    f"0x{plt['addr'] + po:x} (imm13={imm13})")
            # the slot must hold (.PLT0 - call), i.e. -(so + 4)
            got = struct.unpack_from(BE + "q", data, plt["off"] + po)[0]
            if got != -(so + 4):
                errors.append(f"entry {i}: data ptr holds {got}, "
                              f"expected {-(so + 4)}")

    print(f"highest byte written by the layout = {high_water} (0x{high_water:x}); "
          f"sh_size = {plt['size']} -> "
          f"{'OK (fits)' if high_water <= plt['size'] else 'OUT OF BOUNDS'}")

    # .rela.plt targets
    for k in range(nrel):
        o = relplt["off"] + k * 24
        r_offset, r_sym, r_type_data, r_type, r_addend = struct.unpack_from(
            BE + "QI3sBq", data, o)
        if r_type != R_SPARC_JMP_SLOT:
            errors.append(f"rela.plt[{k}]: type {r_type} != R_SPARC_JMP_SLOT")
            continue
        want = plt["addr"] + (to_plt_offset(k) if k < NSMALL
                              else ptr_offset(k, n))
        if r_offset != want:
            errors.append(f"rela.plt[{k}]: r_offset 0x{r_offset:x} != "
                          f"expected 0x{want:x}")
        if not (plt["addr"] <= r_offset < plt["addr"] + plt["size"]):
            errors.append(f"rela.plt[{k}]: r_offset 0x{r_offset:x} outside .plt")

    # does anything else overlap the region the PLT writes into?
    plt_end_file = plt["off"] + plt["size"]
    for s in shdrs:
        if s is plt or s["type"] == 8 or s["size"] == 0:  # SHT_NOBITS
            continue
        if s["off"] < plt_end_file and plt["off"] < s["off"] + s["size"]:
            errors.append(f"section {s['sname']} file range overlaps .plt")

    if errors:
        print(f"\n{len(errors)} PROBLEM(S):")
        for e in errors[:20]:
            print("  " + e)
        sys.exit(1)
    print("\nall PLT layout checks passed")


if __name__ == "__main__":
    main()
