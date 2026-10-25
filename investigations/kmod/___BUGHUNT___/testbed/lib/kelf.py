"""Independent minimal ELF / kernel-module reader used as an oracle against libkmod.

Shares no code with libkmod on purpose: a disagreement between the two is either a libkmod bug
or an oracle bug, and both are worth knowing about.
"""
import gzip
import lzma
import struct
from compression import zstd

SIG_MAGIC = b"~Module signature appended~\n"


def load_module_bytes(path):
    data = open(path, "rb").read()
    if data[:4] == b"\x28\xb5\x2f\xfd":
        return zstd.decompress(data)
    if data[:6] == b"\xfd7zXZ\x00":
        return lzma.decompress(data)
    if data[:2] == b"\x1f\x8b":
        return gzip.decompress(data)
    return data


class Elf:
    def __init__(self, data):
        if data[:4] != b"\x7fELF":
            raise ValueError("not ELF")
        self.data = data
        self.is64 = data[4] == 2
        self.end = "<" if data[5] == 1 else ">"
        e = self.end
        if self.is64:
            shoff, = struct.unpack_from(e + "Q", data, 0x28)
            shentsize, shnum, shstrndx = struct.unpack_from(e + "HHH", data, 0x3A)
        else:
            shoff, = struct.unpack_from(e + "I", data, 0x20)
            shentsize, shnum, shstrndx = struct.unpack_from(e + "HHH", data, 0x2E)
        self.sections = []
        for i in range(shnum):
            o = shoff + i * shentsize
            if self.is64:
                name, typ, flags, addr, off, size, link, info, align, entsize = struct.unpack_from(
                    e + "IIQQQQIIQQ", data, o)
            else:
                name, typ, flags, addr, off, size, link, info, align, entsize = struct.unpack_from(
                    e + "IIIIIIIIII", data, o)
            self.sections.append(dict(name_off=name, type=typ, offset=off, size=size, link=link,
                                      entsize=entsize))
        strtab = self.sections[shstrndx]
        for s in self.sections:
            s["name"] = self._cstr(strtab["offset"] + s["name_off"])

    def _cstr(self, off):
        end = self.data.index(b"\0", off)
        return self.data[off:end].decode("latin-1")

    def section(self, name):
        for s in self.sections:
            if s["name"] == name:
                return s
        return None

    def section_bytes(self, name):
        s = self.section(name)
        if s is None or s["type"] == 8:  # SHT_NOBITS
            return None
        return self.data[s["offset"]:s["offset"] + s["size"]]

    def modinfo(self):
        raw = self.section_bytes(".modinfo")
        if raw is None:
            return []
        out = []
        for item in raw.split(b"\0"):
            if not item:
                continue
            k, _, v = item.decode("utf-8", "replace").partition("=")
            out.append((k, v))
        return out

    def build_id(self):
        """GNU build-id note as libkmod/modinfo print it (AA:BB:...), None if absent."""
        raw = self.section_bytes(".note.gnu.build-id")
        if not raw or len(raw) < 16:
            return None
        namesz, descsz, typ = struct.unpack_from(self.end + "III", raw, 0)
        if typ != 3 or raw[12:12 + namesz] != b"GNU\0":
            return None
        off = 12 + ((namesz + 3) & ~3)
        return ":".join(f"{b:02X}" for b in raw[off:off + descsz])

    def versions(self):
        raw = self.section_bytes("__versions")
        if raw is None:
            return []
        entsz = 64
        out = []
        for i in range(0, len(raw) - entsz + 1, entsz):
            crc, = struct.unpack_from(self.end + ("Q" if self.is64 else "I"), raw, i)
            crc &= 0xFFFFFFFF
            name = raw[i + (8 if self.is64 else 4):i + entsz].split(b"\0")[0].decode("latin-1")
            out.append((name, crc))
        return out

    def symbols(self):
        """(name, shndx, bind) for every .symtab entry."""
        st = self.section(".symtab")
        if st is None:
            return []
        strs = self.sections[st["link"]]
        raw = self.data[st["offset"]:st["offset"] + st["size"]]
        out = []
        esz = 24 if self.is64 else 16
        for i in range(esz, len(raw) - esz + 1, esz):
            if self.is64:
                name, info, other, shndx, value, size = struct.unpack_from(self.end + "IBBHQQ", raw, i)
            else:
                name, value, size, info, other, shndx = struct.unpack_from(self.end + "IIIBBH", raw, i)
            out.append((self._cstr(strs["offset"] + name), shndx, info >> 4))
        return out


def signature_trailer(data):
    """Parse the appended module signature header; None when unsigned."""
    if not data.endswith(SIG_MAGIC):
        return None
    end = len(data) - len(SIG_MAGIC)
    algo, hash_, id_type, signer_len, key_id_len, _p0, _p1, _p2, sig_len = struct.unpack_from(
        ">BBBBBBBBI", data, end - 12)
    sig_start = end - 12 - sig_len
    return dict(id_type=id_type, hash=hash_, signer_len=signer_len, key_id_len=key_id_len,
                sig=data[sig_start:end - 12])
