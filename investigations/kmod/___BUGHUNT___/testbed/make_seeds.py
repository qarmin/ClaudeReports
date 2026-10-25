#!/usr/bin/env python3
"""Build the libFuzzer seed corpora in __corpus-data/seeds/<target>/ (deterministic)."""
import gzip
import lzma
import shutil
import subprocess
import sys
from compression import zstd
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent / "lib"))
import kelf  # noqa: E402
from common import REAL_MODDIR, SRC, TB, real_modules, sample, run  # noqa: E402

S = TB / "__corpus-data" / "seeds"
PLAYGROUND = [TB.parent / "__build-asan" / "testsuite" / "module-playground",
              TB / "__build-fuzz" / "testsuite" / "module-playground"]
PRISTINE = SRC / "testsuite" / "rootfs-pristine"


def put(target, name, data):
    d = S / target
    d.mkdir(parents=True, exist_ok=True)
    (d / name).write_bytes(data)


def main():
    shutil.rmtree(S, ignore_errors=True)
    small = [p for p in real_modules() if p.stat().st_size < 12000]
    reals = sample(small, 40, 1)
    playground = sorted({p.name: p for d in PLAYGROUND if d.exists() for p in d.glob("*.ko")}.values())
    for p in playground:
        put("elf", p.name, p.read_bytes())
        put("modfile", p.name, p.read_bytes())
    for i, p in enumerate(reals):
        raw = kelf.load_module_bytes(p)
        put("elf", f"real{i}.ko", raw)
        put("modfile", f"real{i}.ko.zst", p.read_bytes())
        if i < 10:
            put("modfile", f"real{i}.ko.xz", lzma.compress(raw, check=lzma.CHECK_CRC32))
            put("modfile", f"real{i}.ko.gz", gzip.compress(raw))

    tmp = TB / "__corpus-data" / "seed-depmod"
    shutil.rmtree(tmp, ignore_errors=True)
    run([TB / "__build-fuzz" / "depmod", "-b", REAL_MODDIR.parents[2], "-o", tmp, "7.0.0-38-generic"], timeout=600)
    bins = list((tmp / "lib" / "modules" / "7.0.0-38-generic").glob("*.bin")) + sorted(PRISTINE.rglob("*.bin"))
    keys = [b"", b"e1000e", b"pci:v00008086d000010D3sv*", b"symbol:printk", b"ext4", b"*", b"snd_hda_intel"]
    n = 0
    for b in bins:
        data = b.read_bytes()
        if len(data) > 1000000:
            continue
        for k in keys[: 3 if n > 40 else len(keys)]:
            put("index", f"idx{n}", bytes([len(k)]) + k + data)
            n += 1
    shutil.rmtree(tmp, ignore_errors=True)

    confs = sorted(PRISTINE.rglob("*.conf")) + sorted((REAL_MODDIR.parents[2] / "lib" / "modprobe.d").glob("*.conf"))
    cmdlines = [b"", b"BOOT_IMAGE=/vmlinuz root=/dev/sda1 e1000e.debug=1 modprobe.blacklist=floppy,pcspkr",
                b"foo.bar=\"a b\" baz.x=1 modprobe.blacklist=", b"a.b=c d.e"]
    for i, c in enumerate(confs):
        put("config", f"conf{i}", bytes([len(cmdlines[i % 4])]) + cmdlines[i % 4] + c.read_bytes())
    put("config", "handwritten", bytes([len(cmdlines[1])]) + cmdlines[1] +
        b"alias a b\noptions b x=1 $CMDLINE_OPTS\ninstall b /bin/true $CMDLINE_OPTS\n"
        b"softdep b pre: c d post: e\nweakdep b f g\nblacklist h\nremove b /bin/false\n"
        b"options b \\\n y=2\n# comment\n")

    bi = REAL_MODDIR / "modules.builtin.modinfo"
    for name in [b"ext4", b"vt", b"xhci_hcd", b""]:
        put("builtin", f"real-{name.decode() or 'none'}", bytes([len(name)]) + name + bi.read_bytes())
    for i, f in enumerate(sorted(PRISTINE.rglob("modules.builtin.modinfo"))):
        put("builtin", f"pristine{i}", b"\x04ext4" + f.read_bytes())

    for i, s in enumerate([b"-v", b"-v -n", b"'a b' c", b"x'a b'y", b"--first-time  -q", b"\"", b"a 'b"]):
        put("modprobe_env", f"env{i}", s)
    for t in sorted(S.iterdir()):
        print(f"{t.name:14s} {len(list(t.iterdir()))} seeds")


if __name__ == "__main__":
    main()
