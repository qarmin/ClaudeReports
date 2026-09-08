#!/bin/bash
# Attempts to reproduce the hypothesised SPARC64 large-PLT out-of-bounds write:
# "PltSection<E>::update_shdr sizes .plt at 16 bytes/entry while the large-PLT
#  layout needs 32, so any link with more than sparc_num_small_plt entries
#  writes past the end of .plt".
#
# For each symbol count it links a synthetic SPARC64 object with mold and then
# checks the resulting .plt against the layout mold's own code assumes.
#
# Usage: run.sh /path/to/mold
set -u

MOLD=${1:-}
if [ -z "$MOLD" ]; then
  echo "usage: $0 /path/to/mold" >&2
  exit 2
fi

HERE=$(cd "$(dirname "$0")" && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "mold: $("$MOLD" --version)"
echo

# sparc_num_small_plt == (0x100000 - 128)/32 == 32764.
# Counts around the small/large boundary and around the 160-entry block edges.
COUNTS="100 32763 32764 32765 32766 32923 32924 32925 33083 33084 33085 33100 34000"

fail=0
for n in $COUNTS; do
  obj="$TMP/t$n.o"
  so="$TMP/t$n.so"
  python3 "$HERE/gen_sparc64_plt.py" "$n" "$obj" > /dev/null || { echo "gen failed for $n"; fail=1; continue; }

  out=$("$MOLD" -shared -o "$so" "$obj" 2>&1)
  rc=$?
  if [ $rc -ne 0 ]; then
    echo "=== nsyms=$n : mold FAILED (exit $rc) ==="
    echo "$out"
    fail=1
    rm -f "$obj" "$so"
    continue
  fi

  chk=$(python3 "$HERE/check_plt.py" "$so" 2>&1)
  crc=$?
  large=$(( n > 32764 ? n - 32764 : 0 ))
  if [ $crc -ne 0 ]; then
    echo "=== nsyms=$n (large=$large) : LAYOUT PROBLEM ==="
    echo "$chk"
    fail=1
  else
    echo "nsyms=$n (large=$large): link OK, $(echo "$chk" | grep 'highest byte')"
  fi
  rm -f "$obj" "$so"
done

echo
if [ $fail -eq 0 ]; then
  echo "RESULT: no out-of-bounds write reproduced; .plt sh_size covers the layout exactly in every case."
  exit 0
fi
echo "RESULT: a problem was reproduced (see above)."
exit 1
