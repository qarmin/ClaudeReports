"""make_signed.py <out.ko> <signature.der>: minimal ELF64 module (.modinfo only) + PKCS#7 blob + kernel module_signature trailer."""
import struct
import sys

out, sigfile = sys.argv[1], sys.argv[2]
secs = [(".modinfo", b"license=GPL\0")]
names = [n for n, _ in secs] + [".shstrtab"]
shstr, name_off = b"\0", {}
for n in names:
    name_off[n] = len(shstr)
    shstr += n.encode() + b"\0"
body, placed, off = b"", [], 64
for n, data in secs + [(".shstrtab", shstr)]:
    pad = (-off) % 8
    body += b"\0" * pad
    off += pad
    placed.append((n, off, len(data)))
    body += data
    off += len(data)
pad = (-off) % 8
body += b"\0" * pad
off_sh = off + pad
nsec = len(placed) + 1
ehdr = b"\x7fELF" + bytes([2, 1, 1, 0]) + b"\0" * 8
ehdr += struct.pack("<HHIQQQIHHHHHH", 1, 62, 1, 0, 0, off_sh, 0, 64, 0, 0, 64, nsec, nsec - 1)
shdrs = b"\0" * 64
for n, o, sz in placed:
    shdrs += struct.pack("<IIQQQQIIQQ", name_off[n], 3 if n == ".shstrtab" else 1, 0, 0, o, sz, 0, 0, 1, 0)
ko = ehdr + body + shdrs
sig = open(sigfile, "rb").read()
trailer = struct.pack(">BBBBB3xI", 0, 4, 2, 0, 0, len(sig)) + b"~Module signature appended~\n"
open(out, "wb").write(ko + sig + trailer)
