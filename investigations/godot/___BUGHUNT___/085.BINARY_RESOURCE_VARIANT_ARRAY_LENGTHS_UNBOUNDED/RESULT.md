# `ResourceLoaderBinary::parse_variant()`: Array / Dictionary / packed-array element counts are unbounded (huge allocation; trap when it fails)

Found by the field-overwrite scan (`tmp/bh/scan.py`) of saved `.res` seeds: overwriting a count with 0x7fffffff made ASan report an out-of-memory
abort in `Memory::alloc_static` called from `parse_variant()`.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Field-overwrite scan of an ArrayMesh `.res` and a Gradient+meta `.res` | build with the earlier candidates' fixes, values 0x7fffffff / 0xffffffff / 0x80000000 / 0x1000001 | ASan "out of memory: trying to allocate 0xbffe80048 bytes" at two offsets, hangs at others |
| Generated repro (`repro/project/repro.gd`): Gradient with an `Array` meta of 3 ints, element count patched to 2147483647, loaded with `allocator_may_return_null=1` (the allocation fails like `malloc` does on a normal 29 GB machine) | unfixed build | two "Parameter mem is null" errors, then `FATAL: Index p_index = 0 is out of bounds (size() = 0)` at `resource_format_binary.cpp:486` (`a[i] = val` after the failed `resize`), exit 134, 10/10. Without that option ASan aborts on the 48 GB request instead (exit 1) |
| Fix verification | `patch.diff`, rebuilt | "Invalid length in binary resource.", load returns null, exit 0, 10/10; saved Gradient/ArrayMesh/scene seeds load as before and a scene round-trip is identical |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after. The abort-on-failed-allocation path is environment dependent: where the 48 GB request is granted lazily the
loader instead spends its time constructing 2^31 Variants (hang / memory exhaustion).

**Fully reproducible**: Yes - real `ResourceLoader.load()` of a `.res` written by `ResourceSaver` with one count patched. The `allocator_may_return_null=1`
option is only there to make the sanitizer allocator behave like a normal allocator that returns null.

## Class

`crash` (and denial of service: gigabytes of allocation / billions of element constructions from a ~360 byte file)

## Root cause

`core/io/resource_format_binary.cpp`, `parse_variant()`, the `VARIANT_DICTIONARY`, `VARIANT_ARRAY` and all `VARIANT_PACKED_*_ARRAY` cases read a `uint32_t len` from the
file and immediately `resize(len)` (or loop `len` times) without comparing it with the bytes left. The `VARIANT_ARRAY` case then does `a[i] = val` on an array whose `resize()` failed
(the failure is not checked), which trips the vector bounds trap; packed arrays hand multi-GB uninitialized buffers back to the caller; `len * 2` / `len * 3` / `len * 4` in 32-bit arithmetic
for the real/vector packed arrays can wrap.

## Introduced by

0b806ee0fc GODOT IS OPEN SOURCE (present since the initial public import)

## The fix

`patch.diff` (+12/-0): for each of the 12 cases reject the resource when `len * minimum_element_size` (dictionary 8, array 4, byte 1, int32 4, int64 8, float32 4, float64 8, string 4, Vector2/3/4 `n * sizeof(real_t)`, Color 16)
exceeds the bytes left in the file. Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree. Only the Array case was reproduced end to end; the packed-array and dictionary
cases got the same guard by reading the code, not individual repros. Other loops in this loader (`VARIANT_NODE_PATH` name counts are 16-bit so bounded, object property counts, `VARIANT_OBJECT` internals) were not audited.
The text (`.tres`) and other format loaders were not checked for the same pattern.
