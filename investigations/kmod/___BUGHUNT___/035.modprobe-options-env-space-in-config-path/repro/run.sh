#!/usr/bin/env bash
# run.sh <kmod_src_dir> <build_dir>: modprobe -C "<dir with a space>" runs an install command that itself runs `modprobe -c` (the usual `install foo modprobe --ignore-install foo` pattern). The child learns the config path only through MODPROBE_OPTIONS. Exit 1 = bug reproduced (child does not see the same configuration).
set -u
bld=$(realpath "${2:?build dir}")
ulimit -c 0
export LD_LIBRARY_PATH=$(dirname "$(clang -print-file-name=libclang_rt.asan-x86_64.so)") ASAN_OPTIONS=detect_leaks=0
unset MODPROBE_OPTIONS
root=$(mktemp -d); trap 'rm -rf "$root"' EXIT
mkdir -p "$root/lib/modules/1.0/kernel" "$root/my conf"
touch "$root"/lib/modules/1.0/modules.{order,builtin,builtin.modinfo}
PYTHONPATH="$(dirname "$0")" python3 -c "
import sys; from synth import module
open(sys.argv[1] + '/lib/modules/1.0/kernel/m0.ko', 'wb').write(module(['s0'], []))" "$root"
"$bld/depmod" -b "$root" 1.0 2>/dev/null
cat > "$root/my conf/a.conf" <<CONF
options m0 marker=1
install m0 echo MODPROBE_OPTIONS=[\$MODPROBE_OPTIONS] > $root/env; $bld/modprobe -d $root -S 1.0 -c > $root/child_config 2>&1
CONF
"$bld/modprobe" -d "$root" -S 1.0 -C "$root/my conf" m0 >/dev/null 2>&1
echo "### MODPROBE_OPTIONS seen by the install command:"; sed "s|$root|ROOT|g" "$root/env"
echo "### 'options m0 marker=1' visible to the child modprobe -c:"
grep -c 'marker=1' "$root/child_config"
grep -q 'marker=1' "$root/child_config"
