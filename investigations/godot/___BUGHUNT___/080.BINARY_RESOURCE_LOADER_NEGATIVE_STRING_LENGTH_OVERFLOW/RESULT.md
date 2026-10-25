# `ResourceLoaderBinary::get_unicode_string()` trusts a signed 32-bit string length from the file (crash / heap-buffer-overflow in `fread`)

Found by the GDScript binary-format fuzz loop (`ResourceLoader.load()` of byte-mutated `.res` and `.res` mesh files),
ASan editor build, seed 1. The fuzzer hit a heap-buffer-overflow inside `fread` called from `get_unicode_string()`
(`resource_format_binary.cpp:868`), twice (a Gradient resource and an ArrayMesh resource).

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of binary `.res` (Gradient with meta, ArrayMesh) | ASan editor build `b8420aa3a6`, seed 1, 1000 iterations | 2 ASan heap-buffer-overflow in `fread` from `get_unicode_string()` |
| Generated repro (`repro/project/repro.gd`): save a Gradient to `.res`, set the class-name string length to -1 | unfixed build | crash 10/10 (`FATAL: Index p_index = 0 is out of bounds (size() = 0)`, signal 4) |
| Fix verification | `patch.diff`, rebuilt | `ResourceLoader.load` returns null with "Invalid string length in binary resource." errors, exit 0, 10/10; both fuzz-found files load cleanly (null) instead of crashing |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after.

**Fully reproducible**: Yes - real `ResourceLoader.load()` of a `.res` file written by `ResourceSaver` with one length field corrupted.

## Class

`memory-safety` (negative length converted to a huge `get_buffer` size: heap-buffer-overflow; with a length of -1 the
empty buffer indexing trap fires instead; a huge positive length also allocates up to 2 GB for a few-hundred-byte file)

## Root cause

`core/io/resource_format_binary.cpp`:

```cpp
String ResourceLoaderBinary::get_unicode_string() {
    int len = f->get_32();
    if (len > str_buf.size()) { str_buf.resize(len); }
    if (len == 0) { return String(); }
    f->get_buffer((uint8_t *)&str_buf[0], len);       // len < 0 -> converted to a ~2^64 size
    return String::utf8(&str_buf[0], len);
}
```

`len` is read as a signed `int` from the file and never validated. A value `>= 0x80000000` is negative: it skips the
`resize`, and `get_buffer()` receives it as a huge `uint64_t` (`fread` overflows the old `str_buf`; with an empty `str_buf`
`&str_buf[0]` traps). A large positive length makes `str_buf.resize()` allocate that much and `String::utf8` convert bytes
beyond the real data. `get_ustring()` (static helper in the same file) has the same unvalidated pattern.

## Introduced by

0b806ee0fc GODOT IS OPEN SOURCE (present since the initial public import)

## The fix

`patch.diff` (+2/-0): in both functions reject `len < 0` or `len` greater than the bytes left in the file
(`ERR_FAIL_COND_V_MSG(..., String(), ...)`). Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree.
After the first bad length the loader keeps reading (each following string read also fails, producing ~19 error lines for the
repro) before ending with "Premature end of file" - it ends cleanly, but returning early would be tidier. The same
parser has many other length-prefixed reads (arrays, Variant payloads in `parse_variant()`); only the `.res` mutation
run above was done, so further findings there are possible. Candidate 081 covers a separate crash found by the same run on `.scn`.
