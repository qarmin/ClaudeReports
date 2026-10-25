#!/usr/bin/env bash
# make_api_tree.sh <out_dir> <depmod>: synthetic module tree for fuzz_api (m0..m7 chain, aliases, one builtin).
set -euo pipefail
out=$1; depmod=$2
here=$(cd "$(dirname "$0")" && pwd)
d=$out/lib/modules/1.0
rm -rf "$out"; mkdir -p "$d/kernel/sub"
touch "$d/modules.order" "$d/modules.builtin.modinfo"
echo "kernel/builtin0.ko" > "$d/modules.builtin"
PYTHONPATH="$here/../lib" python3 - "$d/kernel" <<'P'
import sys
from synth import module
k = sys.argv[1]
for i in range(8):
    imp = ['s%d' % (i - 1)] if i else []
    info = ['license=GPL', 'alias=a%d' % (i % 2), 'alias=pci:v1d%d*' % i, 'softdep=pre: m%d' % ((i + 3) % 8), 'parm=p:desc', 'parmtype=p:int']
    open('%s/%sm%d.ko' % (k, 'sub/' if i % 3 == 0 else '', i), 'wb').write(module(['s%d' % i], imp, info))
P
LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") "$depmod" -b "$out" 1.0
