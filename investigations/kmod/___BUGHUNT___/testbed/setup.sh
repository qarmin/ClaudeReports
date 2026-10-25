#!/usr/bin/env bash
# (Re)create the testbed's pristine source snapshot + builds + seed corpora from the main
# project's HEAD. Never touches the main working tree (candidates patch it temporarily).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
proj=$(cd "$here/../.." && pwd)
rm -rf "$here/_src" "$here/__build-fuzz" "$here/__build-gcc" "$here/__build-plain"
mkdir -p "$here/_src"
git -C "$proj" archive HEAD | tar -x -C "$here/_src"
git -C "$proj" rev-parse --short HEAD > "$here/_src/.commit"
opts=(--native-file "$here/_src/build-dev.ini" -Ddocs=false -Dmanpages=false -Dwerror=false)
CC=clang meson setup "$here/__build-fuzz" "$here/_src" "${opts[@]}" -Dc_args=-fsanitize=fuzzer-no-link >/dev/null
ninja -C "$here/__build-fuzz" >/dev/null
CC=gcc meson setup "$here/__build-gcc" "$here/_src" "${opts[@]}" >/dev/null
ninja -C "$here/__build-gcc" >/dev/null
CC=clang meson setup "$here/__build-plain" "$here/_src" -Dbuildtype=debug -Ddocs=false -Dmanpages=false -Dwerror=false -Dmbedtls=disabled -Dbuild-tests=false >/dev/null
ninja -C "$here/__build-plain" >/dev/null
clang -O1 -shared -fPIC "$here/fuzz/failmalloc.c" -ldl -o "$here/__build-plain/failmalloc.so"
"$here/fuzz/build.sh" >/dev/null
"$here/drivers/build.sh" >/dev/null
[ -d "$here/__corpus-data/realroot" ] || "$here/fetch_corpus.sh"
"$here/fuzz/make_api_tree.sh" "$here/__corpus-data/api-tree" "$here/__build-gcc/depmod" >/dev/null
python3 "$here/make_seeds.py"
echo "testbed ready at $(cat "$here/_src/.commit")"
