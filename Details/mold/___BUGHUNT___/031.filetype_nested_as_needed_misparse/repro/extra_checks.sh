#!/bin/bash
# Same-bug-class inputs beyond the ones in repro/run.sh (CONFIRMATION.md section 7):
# deep nesting, degenerate/truncated nesting, and the machine-type inference the
# function actually exists for.
set -u
MOLD=$(readlink -f "${1:?usage: extra.sh /path/to/mold}")
LD=/usr/bin/ld
cd "$(dirname "$0")" || exit 1
W=$PWD/extra-work; rm -rf "$W"; mkdir -p "$W"; cd "$W" || exit 1

printf 'int foo(void) { return 42; }\n' > foo.c
printf 'int foo(void);\nint main(void) { return foo() - 42; }\n' > main.c
cc -fPIC -shared -o libfoo.so foo.c || exit 2
cc -c -o main.o main.c || exit 2

gen() { # $1 = depth
  local n=$1 s="" e="" i
  for ((i=0;i<n;i++)); do s+="AS_NEEDED("; e+=")"; done
  printf 'GROUP(%slibfoo.so%s main.o)\n' "$s" "$e"
}

echo "### deep but valid nesting (mold no -m / mold -l / GNU ld)"
for d in 10 100 500 5000; do
  gen $d > deep$d.ld; cp deep$d.ld libdeep$d.so
  timeout 60 "$MOLD" -o deep$d.A -e main deep$d.ld >a.log 2>&1; a=$?
  timeout 60 "$MOLD" -m elf_x86_64 -o deep$d.C -e main -L . -ldeep$d >c.log 2>&1; c=$?
  timeout 60 "$LD" -o deep$d.ldA -e main deep$d.ld >l.log 2>&1; l=$?
  printf '  depth %-5s mold-A=%-3s mold-C=%-3s ld=%-3s %s\n' "$d" "$a" "$c" "$l" "$(head -c 90 a.log | tr '\n' ' ')"
done

echo
echo "### degenerate / truncated nesting - mold must fail cleanly (exit 1), never hang or crash"
degen=(
  'GROUP(AS_NEEDED(AS_NEEDED()))'
  'GROUP(AS_NEEDED(AS_NEEDED'
  'GROUP(AS_NEEDED('
  'GROUP(AS_NEEDED(AS_NEEDED(AS_NEEDED'
  'GROUP(AS_NEEDED(AS_NEEDED(nonexistent.so)))'
  'INPUT(AS_NEEDED(AS_NEEDED(AS_NEEDED())))'
  'GROUP(AS_NEEDED(AS_NEEDED(AS_NEEDED(libfoo.so main.o'
)
i=0
for s in "${degen[@]}"; do
  i=$((i+1)); printf '%s\n' "$s" > d$i.ld
  timeout 60 "$MOLD" -o d$i.out -e main d$i.ld >d.log 2>&1; rc=$?
  timeout 60 "$LD" -o d$i.ldout -e main d$i.ld >dl.log 2>&1; lrc=$?
  printf '  mold=%-3s ld=%-3s  %-55s | %s\n' "$rc" "$lrc" "$s" "$(head -2 d.log | tail -1 | sed 's/^ *//' | head -c 60)"
done

echo
echo "### unchanged paths (must still work)"
printf 'OUTPUT_FORMAT(elf64-x86-64)\nGROUP(libfoo.so main.o)\n' > of.ld
timeout 60 "$MOLD" -o of.out -e main of.ld >o.log 2>&1; echo "  OUTPUT_FORMAT script: exit=$? $(head -c 80 o.log)"
printf 'GROUP(main.o AS_NEEDED(AS_NEEDED(libfoo.so)))\n' > tail.ld
timeout 60 "$MOLD" -o tail.out -e main tail.ld >o.log 2>&1; echo "  nested AS_NEEDED not first: exit=$? $(head -c 80 o.log)"
printf 'GROUP(\n  AS_NEEDED(\n    AS_NEEDED(\n      libfoo.so ) ) main.o )\n' > ml.ld
timeout 60 "$MOLD" -o ml.out -e main ml.ld >o.log 2>&1; echo "  multi-line nested:         exit=$? $(head -c 80 o.log)"

echo
echo "### machine-type inference actually still works through nesting (i386)"
if cc -m32 -c -o main32.o main.c 2>/dev/null; then
  printf 'GROUP(AS_NEEDED(AS_NEEDED(main32.o)))\n' > m32.ld
  timeout 60 "$MOLD" -o m32.out -e main -r m32.ld >m.log 2>&1
  echo "  mold -r on i386-only nested script: exit=$? $(head -c 100 m.log)"
  file m32.out 2>/dev/null | head -c 120; echo
else
  echo "  skipped: no -m32 support on this host"
fi
