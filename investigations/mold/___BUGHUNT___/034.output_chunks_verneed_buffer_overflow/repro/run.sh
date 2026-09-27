#!/bin/bash
# Reproduction for the .gnu.version_r (VerneedSection<E>::construct) heap
# buffer overflow.
#
#   usage: run.sh /path/to/mold
#
# mold sizes the .gnu.version_r staging buffer as
#   (sizeof(ElfVerneed) + sizeof(ElfVernaux)) * (nsyms + 1)      = 32*(n+1) bytes
# but with `-z pack-relative-relocs` it writes, for every DSO group whose
# soname starts with "libc.so." and which defines a "GLIBC_2.*" version, an
# extra ElfVernaux record for the dummy "GLIBC_ABI_DT_RELR" dependency:
#   16*G + 16*(G_glibc + G + W)                                  = 48*n bytes
# when every symbol comes from a different such DSO. That is more than the
# allocation as soon as there are three or more of them.
#
# Exit status: 0 if every link that should succeed succeeded AND no link
# crashed (i.e. the bug is absent); 1 if any link aborted/crashed.
set -u

MOLD="${1:?usage: run.sh /path/to/mold}"
MOLD="$(cd "$(dirname "$MOLD")" && pwd)/$(basename "$MOLD")"
HERE="$(cd "$(dirname "$0")" && pwd)"

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "mold under test: $MOLD"
"$MOLD" --version
echo

fail=0
for n in 1 2 3 4 8 16; do
  d="$WORK/n$n"
  bash "$HERE/gen_inputs.sh" "$d" "$n" > "$d.build.log" 2>&1 ||
    { echo "N=$n: could not build inputs, see $d.build.log"; exit 2; }

  libs=""
  for i in $(seq 0 $((n - 1))); do
    libs="$libs $d/libc.so.$((i + 2))"
  done

  # nsyms = n, allocated = 32*(n+1), written = 48*n
  alloc=$((32 * (n + 1)))
  need=$((48 * n))

  out="$("$MOLD" -shared -o "$d/out.so" "$d/main.o" $libs \
         -z pack-relative-relocs 2>&1)"
  rc=$?

  printf 'N=%-3d allocated=%-4d written=%-4d exit=%-4d %s\n' \
    "$n" "$alloc" "$need" "$rc" \
    "$(printf '%s' "$out" | head -1 | cut -c1-90)"

  if [ "$rc" -ne 0 ]; then
    fail=1
  fi
done

echo
if [ "$fail" -ne 0 ]; then
  echo "RESULT: mold aborted on at least one input -> bug is PRESENT"
  exit 1
fi
echo "RESULT: every link succeeded -> bug is ABSENT"
exit 0
