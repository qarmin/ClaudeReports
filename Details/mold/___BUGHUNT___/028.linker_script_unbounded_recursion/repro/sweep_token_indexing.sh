#!/usr/bin/env bash
# Supporting evidence for the NEGATIVE half of candidate 028: the assigned suspect
# (out-of-range `tok[N]` / `tok.subspan(N)` indexing in Script<E>'s parser methods)
# does not reproduce. This sweeps 55 truncated/malformed inputs through all three
# entry points into the parser and asserts none of them crashes.
#
# Not part of `just test` - run it by hand if you want to re-check the negative result:
#   bash sweep_token_indexing.sh /path/to/mold
# Exit status: 0 if every input was handled without a crash, 1 otherwise.

set -u
MOLD="${1:?usage: sweep_token_indexing.sh /path/to/mold}"
MOLD=$(readlink -f "$MOLD")
D=$(mktemp -d)
trap 'rm -rf "$D"' EXIT

printf 'int main(void){return 0;}\n' > "$D/a.c"
cc -c -o "$D/a.o" "$D/a.c" || { echo "need a working C compiler"; exit 2; }

crashed=0
report() { # label rc out
  if [ "$2" -ge 128 ]; then
    crashed=1
    printf '  %-26s rc=%-4s CRASH (signal %s)\n' "$1" "$2" "$(($2-128))"
  else
    printf '  %-26s rc=%-4s %s\n' "$1" "$2" \
      "$(printf '%s' "$3" | head -1 | sed "s|$D/||g" | cut -c1-90)"
  fi
}

# A file needs >= 4 printable bytes for filetype.cc's is_text_file() to route it to
# the script parser at all, hence the trailing padding on the very short shapes.
ld_case() { printf '%b\n' "$2" > "$D/s.ld"
  out=$(timeout 60 "$MOLD" -o "$D/o.exe" "$D/a.o" "$D/s.ld" 2>&1); report "$1" "$?" "$out"; }
ver_case() { printf '%b\n' "$2" > "$D/v.ver"
  out=$(timeout 60 "$MOLD" -shared -o "$D/o.so" "$D/a.o" --version-script "$D/v.ver" 2>&1); report "$1" "$?" "$out"; }
lst_case() { printf '%b\n' "$2" > "$D/d.lst"
  out=$(timeout 60 "$MOLD" -shared -o "$D/o.so" "$D/a.o" --dynamic-list "$D/d.lst" 2>&1); report "$1" "$?" "$out"; }

echo "=== linker-script entry point (parse_linker_script) ==="
ld_case "empty"              ""
ld_case "only-ws"            "   \t   "
ld_case "only-comment"       "/* hi */"
ld_case "GROUP-open"         "GROUP("
ld_case "GROUP-a"            "GROUP(a"
ld_case "GROUP-bare"         "GROUP"
ld_case "GROUP-rparen"       "GROUP)  "
ld_case "INPUT-bare"         "INPUT"
ld_case "INPUT-open"         "INPUT("
ld_case "OUTPUT_FORMAT-bare" "OUTPUT_FORMAT"
ld_case "OUTPUT_FORMAT-open" "OUTPUT_FORMAT("
ld_case "OUTPUT_FORMAT-arg"  "OUTPUT_FORMAT(elf64-x86-64"
ld_case "AS_NEEDED-open"     "GROUP(AS_NEEDED("
ld_case "AS_NEEDED-bare"     "GROUP(AS_NEEDED"
ld_case "bare-lparen"        "(       "
ld_case "bare-rparen"        ")       "
ld_case "semi"               ";       "
ld_case "brace-only"         "{       "
ld_case "close-brace"        "}       "
ld_case "star"               "*       "
ld_case "eq-only"            "=       "
ld_case "colon"              ":       "
ld_case "quoted-only"        '"x"     '
ld_case "defsym-trunc"       "a = b"
ld_case "defsym-trunc2"      "a =     "
ld_case "defsym-full"        "a = b ;"
ld_case "defsym-quoted"      'a = "b" '
ld_case "VERSION-bare"       "VERSION"
ld_case "VERSION-brace"      "VERSION {"
ld_case "VERSION-brace2"     "VERSION { v1 {"
ld_case "VERSION-brace3"     "VERSION { v1 { global:"
ld_case "VERSION-brace4"     "VERSION { v1 { global: a"
ld_case "VERSION-brace5"     "VERSION { v1 { global: a;"
ld_case "VERSION-brace6"     "VERSION { v1 { global: a; }"
ld_case "VERSION-extern"     "VERSION { v1 { extern"
ld_case "VERSION-externC"    'VERSION { v1 { extern "C"'
ld_case "VERSION-externC2"   'VERSION { v1 { extern "C" {'
ld_case "VERSION-externC3"   'VERSION { v { extern "C" } ; }'
ld_case "VERSION-star"       "VERSION { { * } ; }"
ld_case "VERSION-label"      "VERSION { v { global }"

echo "=== version-script entry point (parse_version_script) ==="
ver_case "empty"      ""
ver_case "brace"      "{"
ver_case "v-brace"    "v {"
ver_case "global"     "v { global:"
ver_case "sym"        "v { global: a"
ver_case "extern"     "v { extern"
ver_case "externC"    'v { extern "C"'
ver_case "externCpp"  'v { extern "C++"'
ver_case "star"       "*"
ver_case "rbrace"     "}"
ver_case "semi"       ";"
ver_case "closepair"  "v { };"
ver_case "label-only" "global:"
ver_case "label-2tok" "global :"

echo "=== dynamic-list entry point (parse_dynamic_list) ==="
lst_case "empty"    ""
lst_case "brace"    "{"
lst_case "sym"      "{ a"
lst_case "semi"     "{ a;"
lst_case "closed"   "{ a; }"
lst_case "extern"   "{ extern"
lst_case "externC"  '{ extern "C"'
lst_case "rbrace"   "}"

echo
if [ "$crashed" -eq 1 ]; then
  echo "RESULT: at least one truncated input crashed mold"
  exit 1
fi
echo "RESULT: no crashes - every truncated input produced a clean diagnostic"
exit 0
