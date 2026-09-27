#!/bin/bash
# Build N shared libraries whose sonames all start with "libc.so." and whose
# version definitions all start with "GLIBC_2.", plus one relocatable object
# that references one versioned symbol from each of them.
#
# Nothing here is malformed: every file is produced by the system compiler and
# linker. The only unusual thing is that more than one of the DSOs looks like a
# glibc to mold's VerneedSection::construct().
#
#   usage: gen_inputs.sh <outdir> <N>
set -eu

DIR="$1"
N="${2:-8}"
mkdir -p "$DIR"

decls=""
calls=""
for i in $(seq 0 $((N - 1))); do
  soname="libc.so.$((i + 2))"
  printf 'int sym%d(void) { return %d; }\n' "$i" "$i" > "$DIR/lib$i.c"
  printf 'GLIBC_2.%d { global: sym%d; local: *; };\n' "$((i + 1))" "$i" \
    > "$DIR/ver$i.map"
  gcc -shared -fPIC -nostdlib -o "$DIR/$soname" "$DIR/lib$i.c" \
      -Wl,--version-script="$DIR/ver$i.map" -Wl,-soname,"$soname"
  decls="${decls}extern int sym$i(void);
"
  calls="${calls}  n += sym$i();
"
done

cat > "$DIR/main.c" <<EOM
$decls
int entry(void) {
  int n = 0;
$calls
  return n;
}
EOM
gcc -c -fPIC -o "$DIR/main.o" "$DIR/main.c"
