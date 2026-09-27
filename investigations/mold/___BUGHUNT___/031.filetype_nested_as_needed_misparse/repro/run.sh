#!/bin/bash
# Repro for candidate 031: get_script_output_type() unwraps only ONE level of AS_NEEDED.
#
#   usage: bash run.sh /path/to/mold [/path/to/gnu-ld]
#
# A linker script such as
#
#     GROUP(AS_NEEDED(AS_NEEDED(libfoo.so)) main.o)
#
# is valid GNU ld syntax (ld's grammar lets an AS_NEEDED clause appear anywhere an
# input_list item can, including inside another AS_NEEDED). GNU ld links it. mold rejects
# it with "library not found: AS_NEEDED" whenever the file goes through
# Script<E>::get_script_output_type() - i.e. whenever mold has to sniff the script for a
# machine type, which happens in two situations:
#
#   mode A: the script is a positional input and no -m was given (detect_machine_type)
#   mode C: the script is pulled in via -l<name> (open_library -> get_machine_type),
#           which happens even when -m IS given - this is how a real toolchain hits it,
#           because /usr/lib/.../libc.so, libatomic_asneeded.so etc. are text scripts
#           found through -l
#
# mode B (positional script + explicit -m) goes through the *main* parser, read_group(),
# which recurses on AS_NEEDED properly - so it succeeds. That asymmetry is the bug.
#
# Exit status: 0 if mold behaves like GNU ld on every shape (i.e. patched), 1 if any
# divergence is seen (i.e. the bug is present).

set -u
MOLD=$(readlink -f "${1:?usage: run.sh /path/to/mold [/path/to/ld]}")
LD=${2:-/usr/bin/ld}
cd "$(dirname "$0")" || exit 1
W=$PWD/work
rm -rf "$W"; mkdir -p "$W"

# --- inputs -----------------------------------------------------------------
printf 'int foo(void) { return 42; }\n' > "$W/foo.c"
printf 'int foo(void);\nint main(void) { return foo() - 42; }\n' > "$W/main.c"
cc -fPIC -shared -o "$W/libfoo.so" "$W/foo.c" || exit 2
cc -c -o "$W/main.o" "$W/main.c"              || exit 2

# Both linkers resolve a bare name inside a script relative to the cwd, so run from $W
# and keep the script bodies short and readable.
cd "$W" || exit 2
O=libfoo.so
M=main.o

# name  |  script body                     (all are accepted by GNU ld)
shapes=(
  "asn0|GROUP($O $M)"
  "asn1|GROUP(AS_NEEDED($O) $M)"
  "asn2|GROUP(AS_NEEDED(AS_NEEDED($O)) $M)"
  "asn3|GROUP(AS_NEEDED(AS_NEEDED(AS_NEEDED($O))) $M)"
  "asn5|GROUP(AS_NEEDED(AS_NEEDED(AS_NEEDED(AS_NEEDED(AS_NEEDED($O))))) $M)"
  "inp2|INPUT(AS_NEEDED(AS_NEEDED($M)) $O)"
  "cmt2|GROUP(AS_NEEDED( /*c*/ AS_NEEDED($O) ) $M)"
  "qut2|GROUP(AS_NEEDED(AS_NEEDED(\"$O\")) $M)"
)

status=0
printf '%-6s %-24s %-24s %-24s %s\n' shape "mold A (no -m)" "mold B (-m given)" "mold C (-l script)" "GNU ld A / C"
printf '%s\n' "-------------------------------------------------------------------------------------------------------------"

for s in "${shapes[@]}"; do
  name=${s%%|*}; body=${s#*|}
  printf '%s\n' "$body" > "$name.ld"
  cp "$name.ld" "lib$name.so"

  "$MOLD" -o "$name.A" -e main "$name.ld"                    > "$name.A.log" 2>&1; a=$?
  "$MOLD" -m elf_x86_64 -o "$name.B" -e main "$name.ld"      > "$name.B.log" 2>&1; b=$?
  "$MOLD" -m elf_x86_64 -o "$name.C" -e main -L . "-l$name"  > "$name.C.log" 2>&1; c=$?
  "$LD" -o "$name.ldA" -e main "$name.ld"                    > "$name.ldA.log" 2>&1; la=$?
  "$LD" -o "$name.ldC" -e main -L . "-l$name"                > "$name.ldC.log" 2>&1; lc=$?

  f() { [ "$1" = 0 ] && echo "ok" || echo "FAIL(exit $1)"; }
  printf '%-6s %-24s %-24s %-24s %s\n' "$name" "$(f $a)" "$(f $b)" "$(f $c)" "$(f $la) / $(f $lc)"

  # GNU ld accepts every shape here, so any non-zero mold exit is a divergence.
  if [ "$la" != 0 ] || [ "$lc" != 0 ]; then
    echo "    !! GNU ld unexpectedly rejected this shape - repro inputs are wrong, not mold"
    status=1
  fi
  for r in "$a:A" "$b:B" "$c:C"; do
    rc=${r%%:*}; mode=${r##*:}
    if [ "$rc" != 0 ]; then
      status=1
      echo "    mold mode $mode diverges from GNU ld:"
      sed 's/^/      /' "$name.$mode.log"
    fi
  done

  # Accepting the script is not enough - the two linkers must also agree on what the
  # nested AS_NEEDED actually meant, i.e. on the resulting DT_NEEDED set.
  if [ "$c" = 0 ] && [ "$lc" = 0 ]; then
    # Compare basenames only: the two linkers spell the same DT_NEEDED differently
    # ("./libfoo.so" vs "libfoo.so"), which is an unrelated pre-existing difference.
    nd() { readelf -d "$1" 2>/dev/null | sed -n 's/.*NEEDED.*\[\(.*\)\].*/\1/p' |
           sed 's|.*/||' | sort | tr '\n' ' '; }
    mn=$(nd "$name.C"); ln=$(nd "$name.ldC")
    if [ "$mn" != "$ln" ]; then
      echo "    DT_NEEDED MISMATCH: mold=[$mn] ld=[$ln]"
      status=1
    else
      echo "    DT_NEEDED agree: [$mn]"
    fi
  fi
done

echo
echo "GNU ld version: $("$LD" --version | head -1)"
if [ "$status" = 0 ]; then
  echo "RESULT: mold matches GNU ld on all shapes (bug NOT present)."
else
  echo "RESULT: mold diverges from GNU ld (bug present)."
fi
exit $status
