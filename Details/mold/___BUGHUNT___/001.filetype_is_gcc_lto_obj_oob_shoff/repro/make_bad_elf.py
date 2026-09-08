import struct, sys

def make_ehdr(e_shoff, e_shnum, e_shstrndx=0, e_type=1, e_machine=62, e_shentsize=64):
    e_ident = bytes([0x7f,0x45,0x4c,0x46, 2,1,1,0, 0,0,0,0,0,0,0,0])
    rest = struct.pack('<HHIQQQIHHHHHH',
        e_type, e_machine, 1,   # e_version
        0,                      # e_entry
        0,                      # e_phoff
        e_shoff,                # e_shoff
        0,                      # e_flags
        64,                     # e_ehsize
        0, 0,                   # e_phentsize, e_phnum
        e_shentsize, e_shnum, e_shstrndx)
    return e_ident + rest

if __name__ == '__main__':
    out = sys.argv[1]
    e_shoff = int(sys.argv[2], 0)
    e_shnum = int(sys.argv[3])
    e_shstrndx = int(sys.argv[4]) if len(sys.argv) > 4 else 0
    data = make_ehdr(e_shoff, e_shnum, e_shstrndx)
    open(out, 'wb').write(data)
    print(f"wrote {out}: {len(data)} bytes, e_shoff=0x{e_shoff:x} e_shnum={e_shnum} e_shstrndx={e_shstrndx}")
