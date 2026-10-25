# depmod-empty-symbol-name-overread: depmod reads past the end of a symbol name when a module imports a symbol with an empty name

`depmod_symbol_find()` (and `depmod_symbol_add()`, and the System.map loader) strip the architecture symbol prefix with `if (name[0] == depmod->cfg->sym_prefix) name++;`. `cfg->sym_prefix` is `'\0'` unless `-P` is given, so for an empty name the test is true and the pointer is moved one byte past the terminating NUL. `hash_find()` then runs `strlen()` on memory outside the allocation. An empty imported name comes from a module whose `__versions`/`.symtab` contain an undefined symbol with an empty name; `depmod` aborts under ASAN (heap-buffer-overflow READ) when it processes such a module, e.g. any module in the tree being indexed.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `depmod -e -F /dev/null -b <root> 1.0` on a one-module tree built with `synth.py` (`imports=['', 'x']`) (`repro/run.sh`) | HEAD ASAN build, 5 runs | 5/5: heap-buffer-overflow READ of size 8 in `strlen` <- `hash_find` <- `depmod_load_module_dependencies` (depmod.c:1669), allocation in `kmod_module_dependency_symbols_new` |
| Exports with an empty name (`exports=['']`, `['a','']`) | HEAD | no crash (add path stays inside the buffer in this layout), same pattern in code |
| Variations after the fix: imports `.`, `''`, `_`, with and without `-P _`, `-e -F /dev/null` | patched | 0 sanitizer reports |

## Status

**Confirmed** - deterministic, 5/5 on HEAD (`output.txt`).

**Fully reproducible**: **Yes** - the real depmod binary on a real file tree. The `-e` (or a symbol lookup of the module's imports) path is needed to reach `depmod_symbol_find()` with the empty name; a real compiler never emits it, so the input is a crafted module.

## Class

memory-safety (out-of-bounds read)

## Root cause

The comparison with the prefix character does not exclude the NUL terminator, which equals the default prefix `'\0'`.

## Introduced by

64b8b58 "kmod-depmod: initial code (no files generated, untested)"

## The fix

`patch.diff` (+3/-3, tools/depmod.c): require `name[0] != '\0'` before treating it as the prefix at the three places (`depmod_symbol_add`, `depmod_symbol_find`, System.map loader). Verified with the repro (5/5 exit 1 -> 5/5 exit 0) and the variations above. Project test suite (`tools/run_tests.sh`, ASAN/UBSAN): 7/7 OK before and 7/7 OK after.

## What's not done / open questions

- The System.map loader variant (a last line `x y ` without a newline) was only fixed by the same pattern, not separately reproduced.
- Other users of a possibly-empty name were not audited.
