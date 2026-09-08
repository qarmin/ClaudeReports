#!/bin/bash
# MallocOutputFile<E>::close() ignores every write failure (src/mold.h).
#
# mold picks MallocOutputFile whenever the output path is "-" or an existing
# non-regular file (device, FIFO, ...); everything else goes through
# MemoryMappedOutputFile. MallocOutputFile::close() calls
#
#     fwrite(this->buf, this->filesize, 1, fp);
#     fclose(fp);
#
# and checks neither result, so a failing write produces a truncated (or
# entirely unwritten) output binary with exit status 0.
#
# usage: MOLD=/path/to/mold ./run.sh      (default: ../clone/build/mold)
#
# Prints a VERDICT line; always exits 0 so a `just test` cycle runs both the
# pristine and the patched phase to completion.

set -u

here=$(cd "$(dirname "$0")" && pwd)
MOLD=${MOLD:-$here/../clone/build/mold}
CC=${CC:-cc}

if [ ! -x "$MOLD" ]; then
  echo "no mold binary at $MOLD (set MOLD=...)" >&2
  exit 2
fi

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

echo "mold:   $MOLD"
"$MOLD" --version
echo

# A freestanding object so the link needs no crt files or libc.
cat > "$work/mini.c" <<'EOF'
void _start(void) { for (;;); }
EOF
$CC -c -ffreestanding -o "$work/mini.o" "$work/mini.c" || exit 2

link=("$MOLD" -e _start "$work/mini.o")

# ---------------------------------------------------------------- baseline
"${link[@]}" -o - > "$work/full.bin" || { echo "baseline link failed" >&2; exit 2; }
full=$(stat -c%s "$work/full.bin")
echo "baseline: a good link writes $full bytes"
echo

bug=0

# --- 1: output is /dev/full - every write fails with ENOSPC ---------------
echo "== 1. mold -o /dev/full  (MallocOutputFile: every write fails, ENOSPC) =="
"${link[@]}" -o /dev/full
rc=$?
echo "   mold exit status : $rc"
echo "   bytes landed     : 0 (the whole link output was dropped)"
if [ $rc -eq 0 ]; then
  echo "   -> SILENT FAILURE: reported success, produced nothing"
  bug=1
else
  echo "   -> error reported, exit status non-zero"
fi
if command -v ld > /dev/null; then
  ld -e _start -o /dev/full "$work/mini.o"
  echo "   GNU ld exit status: $?  (reference behaviour)"
fi
echo

# --- 2: RLIMIT_FSIZE caps stdout - a partial write, truncated output ------
echo "== 2. mold -o -  with RLIMIT_FSIZE=1024 (MallocOutputFile: short write, EFBIG) =="
python3 "$here/limit.py" 1024 "$work/trunc.bin" "${link[@]}" -o -
rc=$?
got=$(stat -c%s "$work/trunc.bin")
echo "   mold exit status : $rc"
echo "   output file size : $got bytes (should be $full)"
if [ $rc -eq 0 ] && [ "$got" -ne "$full" ]; then
  echo "   -> SILENT FAILURE: reported success, wrote a truncated ELF"
  bug=1
elif [ $rc -ne 0 ]; then
  echo "   -> error reported, exit status non-zero"
fi
echo

# --- 3: same failure on a bigger output, so fwrite (not fclose) is the ----
# --- one that notices; both checks are needed to cover both sizes ---------
echo "== 3. mold -o -  with RLIMIT_FSIZE=1024, output padded past stdio's buffer =="
cat > "$work/big.c" <<'EOF'
const char blob[200000] = { 1 };
void _start(void) { for (;;); }
EOF
if $CC -c -ffreestanding -o "$work/big.o" "$work/big.c" 2>/dev/null; then
  "$MOLD" -e _start -o - "$work/big.o" > "$work/bigfull.bin"
  bigfull=$(stat -c%s "$work/bigfull.bin")
  python3 "$here/limit.py" 1024 "$work/bigtrunc.bin" "$MOLD" -e _start -o - "$work/big.o"
  rc=$?
  got=$(stat -c%s "$work/bigtrunc.bin")
  echo "   mold exit status : $rc"
  echo "   output file size : $got bytes (should be $bigfull)"
  if [ $rc -eq 0 ] && [ "$got" -ne "$bigfull" ]; then
    echo "   -> SILENT FAILURE: reported success, wrote a truncated ELF"
    bug=1
  elif [ $rc -ne 0 ]; then
    echo "   -> error reported, exit status non-zero"
  fi
fi
echo

# --- 4: control - a regular output file uses MemoryMappedOutputFile, ------
# --- whose ftruncate IS checked, so the same failure is reported ----------
echo "== 4. control: mold -o <regular file> with RLIMIT_FSIZE=1024 (MemoryMappedOutputFile) =="
python3 "$here/limit.py" 1024 "$work/sink" "${link[@]}" -o "$work/mmap.bin"
echo "   mold exit status : $?  (expected non-zero both before and after the fix)"
echo

echo "-----------------------------------------------------------------"
if [ $bug -eq 1 ]; then
  echo "VERDICT: BUG PRESENT - mold exited 0 while the output was truncated/absent"
else
  echo "VERDICT: BUG ABSENT - every write failure was reported with a non-zero exit"
fi
exit 0
