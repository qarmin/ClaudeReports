# `ResourceLoaderBinary::_get_string()`: string table index from the file is not range-checked (crash), inline string length unbounded

Found by overwriting every 4-byte aligned field of binary `.res` seeds with large values (`tmp/bh/scan.py`) on a build with the
earlier candidates' fixes applied; the scan reported `FATAL: Index p_index = 2147483647 is out of bounds (size() = 12)`.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Field-overwrite scan of a saved Gradient+Curve `.res` (values 0x7fffffff, 0xffffffff, 0x80000000, 0x1000001) | build with 073-082 fixes | several offsets abort with the index trap; one is a property name index |
| Generated repro (`repro/project/repro.gd`): save a Gradient with a Curve meta resource, set the Curve record's first property name index to 0x7fffffff | unfixed build | `FATAL: Index p_index = 2147483647 is out of bounds (size() = 12)`, exit 134, 10/10 |
| Fix verification | `patch.diff`, rebuilt | "Index id = 2147483647 is out of bounds" error, load returns null, exit 0, 10/10 |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after.

**Fully reproducible**: Yes - real `ResourceLoader.load()` of a `.res` written by `ResourceSaver` with one field patched.

## Class

`crash` (out-of-bounds index into the string table -> process abort; same bug class as an out-of-bounds read)

## Root cause

`core/io/resource_format_binary.cpp`:

```cpp
StringName ResourceLoaderBinary::_get_string() {
    uint32_t id = f->get_32();
    if (id & 0x80000000) { /* inline string of 'len' bytes: str_buf.resize(len) with len up to 2^31-1 */ ... }
    return string_map[id];            // id is any u32 from the file
}
```

`_get_string()` is used for property names, node-path parts, etc. `string_map[id]` indexes a `Vector<StringName>` whose size comes from the file's own
string table; an `id >= string_map.size()` hits the vector's bounds trap. In the inline-string branch the length is only limited to 2^31-1, so a short
file makes `str_buf.resize()` allocate up to 2 GB and the following `get_buffer` leaves the rest of that buffer as stale/uninitialized bytes in the resulting string.

## Introduced by

0b806ee0fc GODOT IS OPEN SOURCE (present since the initial public import)

## The fix

`patch.diff` (+2/-0): `ERR_FAIL_UNSIGNED_INDEX_V(id, string_map.size(), StringName())` before the lookup, and reject an inline length larger than the
bytes left in the file. Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree. The same field-overwrite scan still
showed other offsets of the Gradient+Curve and mesh/scene seeds failing (out-of-memory / hangs in `parse_variant()` array and packed-array sizes): see candidate 085 and
`IDEAS.md`. Other `string_map[...]` / `internal_resources[...]` style lookups in the file were not audited one by one.
