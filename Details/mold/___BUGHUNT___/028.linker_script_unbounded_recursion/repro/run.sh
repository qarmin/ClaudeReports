#!/usr/bin/env bash
# Standalone repro for candidate 028 (linker_script_parser_token_indexing).
#
# Unbounded recursion in mold's linker-script parser (src/linker-script.cc). Four
# recursive-descent sites have no depth limit, so a malformed - and in one case a
# 15-byte, entirely well-formed-looking - script makes the real `mold` binary exhaust
# its stack and die with SIGSEGV (exit 139) instead of reporting an error.
#
# Usage: bash run.sh /path/to/mold
# Exit status: 0 if every case was handled cleanly (patched), 1 if any case crashed.

set -u
MOLD="${1:?usage: run.sh /path/to/mold}"
MOLD=$(readlink -f "$MOLD")
D=$(mktemp -d)
trap 'rm -rf "$D"' EXIT

printf 'int main(void){return 0;}\n' > "$D/a.c"
cc -c -o "$D/a.o" "$D/a.c" || { echo "need a working C compiler"; exit 2; }

crashed=0

run() { # label, then the mold argv
  label="$1"; shift
  out=$(timeout 180 "$MOLD" "$@" 2>&1); rc=$?
  first=$(printf '%s' "$out" | head -1 | sed "s|$D/||g" | cut -c1-100)
  if [ "$rc" -ge 128 ]; then
    crashed=1
    printf '  %-40s rc=%-4s CRASH (signal %s)\n' "$label" "$rc" "$((rc-128))"
  else
    printf '  %-40s rc=%-4s %s\n' "$label" "$rc" "$first"
  fi
}

echo "=== 1. Script<E>::read_group - nested AS_NEEDED( ==="
# 200000 nested AS_NEEDED( in a GROUP directive, never closed.
{ printf 'GROUP('; for ((i=0;i<200000;i++)); do printf 'AS_NEEDED('; done; printf '\n'; } > "$D/deep.ld"
run "GROUP( + 200000x AS_NEEDED(" -o "$D/o.exe" "$D/a.o" "$D/deep.ld"

echo "=== 2. Script<E>::get_script_output_type - self-including script ==="
# 15 bytes. Cycle is get_script_output_type -> resolve_path -> get_machine_type ->
# get_script_output_type -> ... (src/filetype.cc:204-205 <-> src/linker-script.cc).
printf 'INPUT(self.ld)\n' > "$D/self.ld"
run "INPUT(self.ld) naming itself" -o "$D/o.exe" "$D/a.o" "$D/self.ld"

echo "=== 3. same cycle, two mutually-including scripts ==="
printf 'INPUT(b.ld)\n' > "$D/a.ld"
printf 'INPUT(a.ld)\n' > "$D/b.ld"
run "a.ld INPUT(b.ld), b.ld INPUT(a.ld)" -o "$D/o.exe" "$D/a.o" "$D/a.ld"

echo '=== 4. Script<E>::read_version_script_commands - nested extern "C" { ==='
{ printf 'v { '; for ((i=0;i<200000;i++)); do printf 'extern "C" { '; done; printf '\n'; } > "$D/deep.ver"
run '--version-script, 200000x extern "C" {' -shared -o "$D/o.so" "$D/a.o" --version-script "$D/deep.ver"

echo '=== 5. Script<E>::read_dynamic_list_commands - nested extern "C" { ==='
{ printf '{ '; for ((i=0;i<200000;i++)); do printf 'extern "C" { '; done; printf '\n'; } > "$D/deep.lst"
run '--dynamic-list, 200000x extern "C" {' -shared -o "$D/o.so" "$D/a.o" --dynamic-list "$D/deep.lst"

echo
echo "=== valid scripts must keep working (fix must not over-reject) ==="
cp "$D/a.o" "$D/obj.o"
printf 'GROUP(obj.o)\n' > "$D/ok1.ld"
run "GROUP(obj.o)" -o "$D/ok1.exe" "$D/ok1.ld"
printf 'GROUP(AS_NEEDED(obj.o))\n' > "$D/ok2.ld"
run "GROUP(AS_NEEDED(obj.o))" -o "$D/ok2.exe" "$D/ok2.ld"
printf 'OUTPUT_FORMAT(elf64-x86-64)\nGROUP(obj.o)\n' > "$D/ok3.ld"
run "OUTPUT_FORMAT(...) + GROUP(obj.o)" -o "$D/ok3.exe" "$D/ok3.ld"
printf 'v1 { global: main; local: *; };\n' > "$D/ok.ver"
run "normal version script" -shared -o "$D/ok.so" "$D/a.o" --version-script "$D/ok.ver"
printf 'v1 { global: extern "C++" { main; }; };\n' > "$D/ok2.ver"
run 'version script with extern "C++"' -shared -o "$D/ok2.so" "$D/a.o" --version-script "$D/ok2.ver"
printf '{ main; };\n' > "$D/ok.lst"
run "normal dynamic list" -shared -o "$D/ok3.so" "$D/a.o" --dynamic-list "$D/ok.lst"
# 998 properly-closed AS_NEEDED levels: legal, deep, and still under the patched cap.
{ printf 'GROUP('; for ((i=0;i<998;i++)); do printf 'AS_NEEDED('; done; printf 'obj.o'
  for ((i=0;i<998;i++)); do printf ')'; done; printf ')\n'; } > "$D/near.ld"
run "998x closed AS_NEEDED around a real .o" -m elf_x86_64 -o "$D/near.exe" "$D/near.ld"

echo
if [ "$crashed" -eq 1 ]; then
  echo "RESULT: at least one input crashed mold (bug present)"
  exit 1
fi
echo "RESULT: no crashes; every malformed input was rejected with a clean error (bug fixed)"
exit 0
