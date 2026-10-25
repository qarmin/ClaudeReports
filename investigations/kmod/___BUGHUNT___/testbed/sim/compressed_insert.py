#!/usr/bin/env python3
"""modprobe of plain/xz/gz/zst modules (kernel without matching compression => userspace decompression + init_module) under sysfake: the memory handed to init_module must equal the original ELF (or differ only by --force-* stripping)."""
import os, subprocess, sys, tempfile, lzma, gzip
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, here + '/../lib')
from compression import zstd
from synth import module

B = sys.argv[1]
S = here + '/sysfake.so'
import struct
def set_alloc(elf):
    """set SHF_ALLOC on every section of a 64-bit LE ELF made by synth (which writes flags 0)"""
    b = bytearray(elf)
    shoff, = struct.unpack_from('<Q', b, 0x28)
    shnum, = struct.unpack_from('<H', b, 0x3c)
    for i in range(1, shnum):
        struct.pack_into('<Q', b, shoff + i * 64 + 8, 2)
    return bytes(b)
mod = set_alloc(module(['sm'], ['sx'], ['license=GPL', 'vermagic=7.0.0 SMP mod_unload', 'depends=']))
bad = 0
for ext, comp in (('', lambda b: b), ('.xz', lzma.compress), ('.gz', gzip.compress), ('.zst', zstd.compress)):
    for extra in ([], ['--force-vermagic'], ['--force-modversion'], ['-f']):
        root = tempfile.mkdtemp(); d = root + '/lib/modules/1.0'; k = d + '/kernel'
        os.makedirs(k); os.makedirs(root + '/fake/proc')
        for f in ('modules.order', 'modules.builtin', 'modules.builtin.modinfo'):
            open(f'{d}/{f}', 'wb').close()
        open(f'{k}/m.ko{ext}', 'wb').write(comp(mod))
        open(f'{k}/x.ko', 'wb').write(module(['sx'], []))
        env = dict(os.environ, SYSFAKE=root + '/fake', LD_PRELOAD=S, ASAN_OPTIONS='detect_leaks=0:verify_asan_link_order=0')
        subprocess.run([B + '/depmod', '-b', root, '1.0'], capture_output=True, env=dict(os.environ, ASAN_OPTIONS='detect_leaks=0'))
        p = subprocess.run([B + '/modprobe', '-d', root, '-S', '1.0'] + extra + ['m'], env=env, capture_output=True, text=True)
        dumps = sorted(f for f in os.listdir(root + '/fake') if f.startswith('init_'))
        res = []
        for f in dumps:
            data = open(root + '/fake/' + f, 'rb').read()
            res.append('same' if data == mod else ('diff:%d bytes' % sum(a != b for a, b in zip(data, mod))) if len(data) == len(mod) else 'len %d vs %d' % (len(data), len(mod)))
        # expected differences: vermagic string (29 bytes) for --force-vermagic/-f, one flag byte (__versions SHF_ALLOC) for --force-modversion/-f
        exp = {(): 0, ('--force-vermagic',): 29, ('--force-modversion',): 1, ('-f',): 30}[tuple(extra)]
        if ext and res and res[0] != ('same' if exp == 0 else 'diff:%d bytes' % exp):
            print('   UNEXPECTED, wanted', exp, 'differing bytes')
            bad += 1
        ok = p.returncode == 0 and (ext == '' or len(res) == 1)
        print(f"{ext or 'plain':5s} {' '.join(extra) or '-':18s} rc={p.returncode} init_module dumps={res} {p.stderr.strip()[:80]}")
        bad += not ok
sys.exit(1 if bad else 0)
