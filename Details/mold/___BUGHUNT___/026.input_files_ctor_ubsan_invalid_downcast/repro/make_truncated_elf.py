import struct, sys

# Writes a truncated x86_64 ET_REL ELF object: valid e_ident + e_type + e_machine
# (enough for filetype.cc's get_file_type() to classify it as FileType::ELF_OBJ and
# route it into ObjectFile<E>'s constructor), but the file is deliberately shorter
# than sizeof(ElfEhdr<E>) (64 bytes), so InputFile<E>::InputFile()'s very first check
# ("if (mf->size < sizeof(ElfEhdr<E>)) Fatal(ctx) << *this << ...") fires while *this
# is still mid-construction (the ObjectFile<E>-derived part has not been constructed
# yet).
#
# Any bytes get_file_type()'s is_gcc_lto_obj<X86_64>() reads past this truncated
# file's real end (e_shoff at offset 40, e_shnum at offset 60) land in the same
# mmap'd page, which the kernel zero-fills - so they read as e_shoff=0, e_shnum=0,
# an empty section-header span, and is_gcc_lto_obj() safely returns false.

def make_truncated_ehdr(total_len, e_type=1, e_machine=62):
    e_ident = bytes([0x7f, 0x45, 0x4c, 0x46, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    rest = struct.pack('<HH', e_type, e_machine)
    data = e_ident + rest
    assert len(data) == 20
    if total_len < len(data):
        data = data[:total_len]
    else:
        data = data + b"\x00" * (total_len - len(data))
    return data

if __name__ == '__main__':
    out = sys.argv[1]
    total_len = int(sys.argv[2]) if len(sys.argv) > 2 else 20
    data = make_truncated_ehdr(total_len)
    open(out, 'wb').write(data)
    print(f"wrote {out}: {len(data)} bytes (sizeof(Elf64_Ehdr)=64, so this is 'too small')")
