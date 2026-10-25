# `ResourceLoaderBinary::open()`: string table / external / internal resource counts are trusted, a tiny `.res` makes the loader loop for billions of iterations

Found by the GDScript binary-format fuzz loop (`ResourceLoader.load()` on byte-mutated `.res`) run against a build that already
contained the fix for candidate 080: the fuzz iteration never finished (90 s timeout, 12 s user + 18 s sys CPU in 30 s - huge
allocations and loops).

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of binary `.res` (build with 073-081 fixes applied) | ASan editor build, seed 1 | one mutated 517-byte file never finishes loading |
| Generated repro (`repro/project/repro.gd`): save a Gradient `.res`, set the string table size to 2147483647 | unfixed build | still running after the 20 s timeout, 10/10 runs |
| Fix verification | `patch.diff`, rebuilt | load fails in 0.6 s with "Table size ... exceeds the file size", exit 0 |
| Overwrite every 4-byte aligned field of a saved Gradient `.res` with 0x7fffffff, load each (fixed build) | 74 files | 9 offsets still abort/fail (rc -6 / 1), several are probably candidate 080's bug (this binary had only the 082 fix) and others other length fields; not analyzed here |

## Status

**Confirmed.** Deterministic: the unfixed build did not return within 20 s in every run; the fixed build returns immediately.
(The unfixed run is bounded only by the timeout - the loop is ~2^31 iterations plus a 16 GB `resize`.)

**Fully reproducible**: Yes - real `ResourceLoader.load()` of a `.res` written by `ResourceSaver` with one header count patched.

## Class

`logic` (denial of service: unbounded loop / huge allocation driven by an unvalidated count in a ~300 byte file)

## Root cause

`core/io/resource_format_binary.cpp`, `ResourceLoaderBinary::open()` reads `string_table_size`, `ext_resources_size` and
`int_resources_size` as `uint32_t` and immediately does `string_map.resize(string_table_size)` and loops that many times
(`get_unicode_string()` and `push_back()` per iteration), without comparing the count with the bytes left in the file. At EOF
`get_32()` returns 0, so each iteration is cheap and the loop just runs for up to 4 billion iterations (the string table also
resizes a `Vector<StringName>` to up to 32 GB first).

## Introduced by

0b806ee0fc GODOT IS OPEN SOURCE (present since the initial public import)

## The fix

`patch.diff` (+15/-0): before each of the three loops, reject the file when `count * minimum_entry_size` (4 bytes per string,
8 per external resource, 12 per internal resource) exceeds the remaining file size (`ERR_FILE_CORRUPT`, file closed). The check has to
sit in the function body (an `ERR_FAIL_MSG` in a helper lambda would only leave the lambda; the first attempt did that and crashed
on the closed file). Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073). Repro on the fixed build: 10/10 clean (exit 0).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree. The second reader of the
same tables in `ResourceFormatLoaderBinary::rename_dependencies()` (same `string_table_size` / `ext_resources_size` / `int_resources_size`
pattern) was not changed or tested. Further per-field problems after a bad length (see the table row about the 9 failing offsets) remain
to be analyzed, they may be separate candidates.
